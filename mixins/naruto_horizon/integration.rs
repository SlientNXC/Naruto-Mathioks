use super::{
    Game,
    naruto_core::{CastError, Character, Jutsu, look},
};
use horizon_client::{
    extras::Corner,
    extras_hud::Corners,
    hud::HudBatch,
    mobs::Damage,
    projectiles::{Projectile, ProjectileKind},
    text::Text,
};
use horizon_core::geometry::Vec3;
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

pub(super) const ID: &str = "naruto_horizon";

#[derive(Default)]
struct Store {
    world: Option<PathBuf>,
    characters: BTreeMap<String, Character>,
    writable: bool,
    ticks: u32,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();
fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| Mutex::new(Store::default()))
}
fn key(uuid: u128) -> String {
    format!("{uuid:032x}")
}

fn save(state: &Store) -> Result<(), String> {
    if !state.writable {
        return Ok(());
    }
    let Some(world) = &state.world else {
        return Ok(());
    };
    let dir = world.join("naruto-horizon");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("characters.json");
    let pending = dir.join("characters.pending.json");
    let bytes = serde_json::to_vec_pretty(&state.characters).map_err(|e| e.to_string())?;
    fs::write(&pending, bytes).map_err(|e| e.to_string())?;
    if path.is_file() {
        fs::copy(&path, dir.join("characters.previous.json")).map_err(|e| e.to_string())?;
    }
    fs::rename(pending, path).map_err(|e| e.to_string())
}

pub(super) fn lifecycle(game: &mut Game<'_>, opened: bool, closed: bool) {
    if opened && !game.native_on(ID) {
        *store().lock().unwrap_or_else(|e| e.into_inner()) = Store::default();
        return;
    }
    let mut state = store().lock().unwrap_or_else(|e| e.into_inner());
    if opened {
        let world = game.native_world_dir();
        let file = world
            .as_ref()
            .map(|p| p.join("naruto-horizon/characters.json"));
        let loaded = file
            .filter(|p| p.exists())
            .map(fs::read)
            .transpose()
            .map_err(|e| e.to_string())
            .and_then(|bytes| {
                bytes
                    .map(|b| serde_json::from_slice::<BTreeMap<String, Character>>(&b))
                    .transpose()
                    .map_err(|e| e.to_string())
            });
        match loaded {
            Ok(characters) => {
                *state = Store {
                    world,
                    characters: characters.unwrap_or_default(),
                    writable: true,
                    ticks: 0,
                };
                for character in state.characters.values_mut() {
                    character.normalize();
                }
            }
            Err(error) => {
                log::error!(
                    "Naruto Horizon did not load its character save: {error}. Original file kept."
                );
                *state = Store {
                    world,
                    ..Store::default()
                };
            }
        }
    }
    if closed {
        if let Err(error) = save(&state) {
            log::error!("Naruto Horizon save failed: {error}");
        }
        *state = Store::default();
    }
}

pub(super) fn tick(game: &mut Game<'_>) {
    if !game.native_on(ID) {
        return;
    }
    let players = game.native_server_players();
    let mut messages = Vec::new();
    {
        let mut state = store().lock().unwrap_or_else(|e| e.into_inner());
        if state.world != game.native_world_dir() {
            return;
        }
        for player in players {
            if let Some(character) = state.characters.get_mut(&key(player.uuid))
                && character.tick()
            {
                messages.push((
                    player.uuid,
                    format!("Level {}! +1 skill point. /naruto spend 1", character.level),
                ));
            }
        }
        state.ticks = state.ticks.wrapping_add(1);
        if state.ticks.is_multiple_of(100)
            && let Err(error) = save(&state)
        {
            log::error!("Naruto Horizon save failed: {error}");
        }
    }
    for (uuid, message) in messages {
        game.native_tell(uuid, Text::literal(message));
    }
}

pub(super) fn mods(mods: &mut Vec<super::native_hooks::NativeMod>) {
    mods.push(super::native_hooks::NativeMod {
        id: ID.into(), title: "Naruto Horizon 0.1.0 — unfinished prototype".into(),
        description: "Independent adaptation of a small part of Mathioks' original Naruto mod. Chakra, progression and three jutsu prototypes. Story, clans and most content are not ported. Single-player only. /naruto help".into(),
        keys: vec![
            ["start".into(), "Begin / character status".into(), "key.keyboard.b".into()],
            ["charge".into(), "Toggle chakra charging".into(), "key.keyboard.r".into()],
            ["cast".into(), "Use selected jutsu".into(), "key.keyboard.j".into()],
            ["next".into(), "Next jutsu".into(), "key.keyboard.n".into()],
        ],
    });
}

pub(super) fn register(commands: &mut Vec<super::native_server::NativeCommand>) {
    commands.push(super::native_server::NativeCommand::new(
        "naruto [action:text]",
    ));
}

pub(super) fn input(game: &mut Game<'_>, input: &str, pressed: bool) {
    if !pressed || !game.native_on(ID) || !game.native_playing() || !game.has_singleplayer_server()
    {
        return;
    }
    for (name, default, action) in [
        ("start", "key.keyboard.b", "start"),
        ("charge", "key.keyboard.r", "charge"),
        ("cast", "key.keyboard.j", "cast"),
        ("next", "key.keyboard.n", "next"),
    ] {
        if input == game.native_mod_key(ID, name, default) {
            game.send_chat(&format!("/naruto {action}"));
            break;
        }
    }
}

pub(super) fn hud(game: &mut Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    if !game.native_on(ID)
        || !game.has_singleplayer_server()
        || game.native_player().is_none()
        || !game.native_corner_shown(Corner::TopLeft)
    {
        return;
    }
    let Some(remote) = &game.remote else {
        return;
    };
    let character = {
        let state = store().lock().unwrap_or_else(|e| e.into_inner());
        state.characters.get(&key(remote.profile())).cloned()
    };
    let Some(character) = character else {
        let (x, y) = corners.place(Corner::TopLeft, 190.0, 28.0);
        game.native_fill(batch, [x, y, 190.0, 28.0], 0xB0101825);
        game.native_text(
            batch,
            "NARUTO HORIZON  /  PROTOTYPE",
            [x + 5.0, y + 4.0],
            0xFFC56A,
            true,
        );
        game.native_text(
            batch,
            "B: begin   /naruto help",
            [x + 5.0, y + 16.0],
            0xFFFFFF,
            true,
        );
        return;
    };
    let (x, y) = corners.place(Corner::TopLeft, 215.0, 68.0);
    game.native_fill(batch, [x, y, 215.0, 68.0], 0xC0101825);
    game.native_text(
        batch,
        &format!(
            "SHINOBI  /  Lv {}   SP {}",
            character.level, character.skill_points
        ),
        [x + 5.0, y + 4.0],
        0xFFC56A,
        true,
    );
    let fraction = character.chakra as f32 / character.ninjutsu.max(1) as f32;
    game.native_fill(batch, [x + 5.0, y + 17.0, 205.0, 6.0], 0xFF25364D);
    game.native_fill(
        batch,
        [x + 5.0, y + 17.0, 205.0 * fraction, 6.0],
        0xFF4AB9F1,
    );
    game.native_text(
        batch,
        &format!(
            "Chakra {}/{}{}",
            character.chakra,
            character.ninjutsu,
            if character.charging { "  CHARGING" } else { "" }
        ),
        [x + 5.0, y + 27.0],
        0xBFE9FF,
        true,
    );
    game.native_text(
        batch,
        character.selected.name(),
        [x + 5.0, y + 39.0],
        0xFFFFFF,
        true,
    );
    game.native_text(
        batch,
        "J: use  N: next  R: charge  B: status",
        [x + 5.0, y + 53.0],
        0xACB8CC,
        true,
    );
}

fn answer(game: &mut Game<'_>, message: impl AsRef<str>) {
    game.native_feedback(Text::literal(message.as_ref()));
}

pub(super) fn command(
    game: &mut Game<'_>,
    player: &super::native_server::NativeServerPlayer,
    line: &str,
    ran: &mut bool,
) {
    let words: Vec<_> = line.split_whitespace().collect();
    if words.first() != Some(&"naruto") || !game.native_on(ID) {
        return;
    }
    *ran = true;
    // This first development slice is limited to the integrated server.
    if game.options.dedicated.is_some() || game.native_server_players().len() > 1 {
        answer(
            game,
            "This Naruto Horizon prototype supports single-player worlds only.",
        );
        return;
    }
    let action = words.get(1).copied().unwrap_or("help");
    if action == "help" {
        answer(
            game,
            "Naruto Horizon 0.1.0: unfinished native prototype, not the full Mathioks mod.",
        );
        answer(
            game,
            "/naruto start | status | charge | next | cast [fireball|rasengan|chidori]",
        );
        answer(
            game,
            "/naruto train (5 chakra, 10s cooldown) | spend <skill points>",
        );
        answer(
            game,
            "Training and open access to these three jutsu are temporary prototype mechanics.",
        );
        return;
    }
    let uuid = key(player.uuid);
    let mut character = {
        let mut state = store().lock().unwrap_or_else(|e| e.into_inner());
        if !state.writable || state.world.is_none() || state.world != game.native_world_dir() {
            answer(
                game,
                "Character save unavailable. See the game log; existing files have been kept.",
            );
            return;
        }
        if action == "start" {
            state.characters.entry(uuid.clone()).or_default();
        }
        state.characters.get(&uuid).cloned()
    };
    let Some(mut character) = character.take() else {
        answer(game, "Use /naruto start to create your character.");
        return;
    };
    match action {
        "start" | "status" => answer(
            game,
            format!(
                "Level {} | Chakra {}/{} | Ninjutsu {} | SP {} | JP {} | XP {}/{}",
                character.level,
                character.chakra,
                character.ninjutsu,
                character.ninjutsu,
                character.skill_points,
                character.jutsu_points,
                character.experience,
                character.xp_needed()
            ),
        ),
        "charge" => {
            character.charging = !character.charging;
            answer(
                game,
                if character.charging {
                    "Charging chakra. R toggles charging off."
                } else {
                    "Chakra charging stopped."
                },
            );
        }
        "next" => {
            character.next_jutsu();
            answer(
                game,
                format!(
                    "{} — {} chakra",
                    character.selected.name(),
                    character.selected.cost(character.ninjutsu)
                ),
            );
        }
        "train" => {
            if character.train() {
                answer(game, "Training complete: +1 chakra experience.");
            } else {
                answer(
                    game,
                    "Training needs 5 chakra and a 10-second rest between attempts.",
                );
            }
        }
        "spend" => {
            if let Some(points) = words.get(2).and_then(|w| w.parse::<u32>().ok())
                && character.spend_ninjutsu(points)
            {
                answer(
                    game,
                    format!("Ninjutsu increased to {}.", character.ninjutsu),
                );
            } else {
                answer(
                    game,
                    "Use /naruto spend <1..1000>, within your available skill points.",
                );
            }
        }
        "demo" => {
            if player.permission < 2 || player.game_mode != 1 {
                answer(
                    game,
                    "Demo setup is available only to an operator in a Creative test world.",
                );
                return;
            }
            character.ninjutsu = 600;
            character.chakra = 600;
            character.cooldown_ticks = 0;
            answer(
                game,
                "Prototype test setup: 600 Ninjutsu and chakra. This changes this character permanently.",
            );
        }
        "cast" => {
            let jutsu = match words.get(2) {
                Some(word) => match Jutsu::parse(word) {
                    Some(j) => j,
                    None => {
                        answer(game, "Choose fireball, rasengan or chidori.");
                        return;
                    }
                },
                None => character.selected,
            };
            if let Err(error) = character.can_cast(jutsu) {
                match error {
                    CastError::Cooldown(ticks) => {
                        answer(game, format!("Wait {:.1}s.", ticks as f32 / 20.0))
                    }
                    CastError::Chakra { have, need } => answer(
                        game,
                        format!(
                            "Need {need} chakra; you have {have}. Increase Ninjutsu if your capacity is too low."
                        ),
                    ),
                }
                return;
            }
            let Some(view) = game.native_player() else {
                return;
            };
            let direction = look(view.yaw, view.pitch);
            if jutsu == Jutsu::Fireball {
                // Horizon's authoritative projectile physics and tracking handle this entity.
                // Original fireball damage scaling and exact explosion parity remain unported.
                let position = Vec3::new(
                    view.x + direction[0] * 1.5,
                    view.y + 1.4 + direction[1] * 1.5,
                    view.z + direction[2] * 1.5,
                );
                let mut projectile = Projectile::fireball(
                    ProjectileKind::LargeFireball { power: 1 },
                    position,
                    Some(horizon_client::mobs::Who::Player),
                    direction,
                );
                projectile.guest = game.lan.as_ref().and_then(|h| {
                    h.hub
                        .guests
                        .iter()
                        .find(|g| g.uuid == player.uuid)
                        .map(|g| g.conn)
                });
                game.add_projectile(projectile);
            } else {
                let Some((index, distance)) = game.mob_hit() else {
                    answer(game, "Aim at a nearby mob. No chakra spent.");
                    return;
                };
                if !game.before_block(distance) {
                    answer(game, "A block is in the way. No chakra spent.");
                    return;
                }
                let damage = Damage::player(player.position);
                let mob = &mut game.mobs[index];
                let id = mob.id;
                if !mob.damage(jutsu.damage(character.ninjutsu) as f32, damage) {
                    answer(game, "That target cannot be damaged now. No chakra spent.");
                    return;
                }
                if jutsu == Jutsu::Rasengan {
                    mob.velocity.x += direction[0] * 1.4;
                    mob.velocity.y += 0.3;
                    mob.velocity.z += direction[2] * 1.4;
                }
                game.resolve_player_responsible(id, damage);
            }
            let _ = character.commit_cast(jutsu);
            answer(game, format!("{}!", jutsu.name()));
        }
        _ => {
            answer(game, "Unknown Naruto command. Use /naruto help.");
            return;
        }
    }
    let mut state = store().lock().unwrap_or_else(|e| e.into_inner());
    state.characters.insert(uuid, character);
    if let Err(error) = save(&state) {
        log::error!("Naruto Horizon save failed: {error}");
        answer(
            game,
            "Character changed, but saving failed. See the game log before quitting.",
        );
    }
}
