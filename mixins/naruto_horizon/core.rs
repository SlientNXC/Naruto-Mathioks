// Independently written mechanics for an unfinished native Horizon adaptation.
// The original mod and its assets are not included. See COVERAGE.md.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Jutsu {
    Fireball,
    Rasengan,
    Chidori,
}

impl Jutsu {
    pub const ALL: [Self; 3] = [Self::Fireball, Self::Rasengan, Self::Chidori];

    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "fireball" => Some(Self::Fireball),
            "rasengan" => Some(Self::Rasengan),
            "chidori" => Some(Self::Chidori),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Fireball => "Fire Release: Fireball",
            Self::Rasengan => "Rasengan",
            Self::Chidori => "Chidori",
        }
    }

    pub fn cost(self, ninjutsu: u32) -> u32 {
        match self {
            Self::Fireball => 10 + ninjutsu / 600,
            Self::Rasengan => 25 + ninjutsu / 300,
            Self::Chidori => 100 + ninjutsu / 600,
        }
    }

    pub fn damage(self, ninjutsu: u32) -> u32 {
        match self {
            Self::Fireball => 6 + ninjutsu / 600,
            Self::Rasengan => 8 + ninjutsu / 300,
            Self::Chidori => 25 + ninjutsu / 60,
        }
    }

    /// Prototype spam guard; not an original-mod parity claim.
    pub fn cooldown(self) -> u32 {
        match self {
            Self::Fireball => 30,
            Self::Rasengan => 20,
            Self::Chidori => 30,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Character {
    pub level: u32,
    pub experience: u32,
    pub skill_points: u32,
    pub jutsu_points: u32,
    pub ninjutsu: u32,
    pub chakra: u32,
    pub selected: Jutsu,
    #[serde(skip)]
    pub regen_ticks: u32,
    #[serde(skip)]
    pub cooldown_ticks: u32,
    #[serde(skip)]
    pub training_ticks: u32,
    #[serde(skip)]
    pub charge_ticks: u32,
    #[serde(skip)]
    pub charging: bool,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            level: 0,
            experience: 0,
            skill_points: 0,
            jutsu_points: 0,
            ninjutsu: 25,
            chakra: 25,
            selected: Jutsu::Fireball,
            regen_ticks: 0,
            cooldown_ticks: 0,
            training_ticks: 0,
            charge_ticks: 0,
            charging: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastError {
    Cooldown(u32),
    Chakra { have: u32, need: u32 },
}

impl Character {
    pub fn normalize(&mut self) {
        self.ninjutsu = self.ninjutsu.clamp(25, 1_000_000);
        self.chakra = self.chakra.min(self.ninjutsu);
        self.level = self.level.min(1_000_000);
    }

    pub fn xp_needed(&self) -> u32 {
        1 + self.level / 4
    }

    /// Original progression: one level at most per authoritative tick.
    pub fn tick(&mut self) -> bool {
        self.cooldown_ticks = self.cooldown_ticks.saturating_sub(1);
        self.training_ticks = self.training_ticks.saturating_sub(1);
        self.regen_ticks = self.regen_ticks.saturating_sub(1);
        if self.regen_ticks == 0 {
            if self.chakra < self.ninjutsu {
                self.regen_ticks = 100;
            }
            self.chakra = self.chakra.saturating_add(1).min(self.ninjutsu);
        }
        // Charging is a prototype input mechanism, separate from passive regen.
        if self.charging {
            self.charge_ticks += 1;
            if self.charge_ticks >= 10 {
                self.charge_ticks = 0;
                self.chakra = self.chakra.saturating_add(1).min(self.ninjutsu);
            }
        }
        let needed = self.xp_needed();
        if self.experience < needed || self.level >= 1_000_000 {
            return false;
        }
        self.experience -= needed;
        self.level += 1;
        self.skill_points = self.skill_points.saturating_add(1);
        let reward = match self.level {
            0..500 => 1,
            500..1000 => 2,
            1000..1500 => 3,
            1500..2000 => 4,
            _ => 5,
        };
        let bonus = if self.level.is_multiple_of(50) { 5 } else { 0 };
        self.jutsu_points = self.jutsu_points.saturating_add(reward + bonus);
        true
    }

    pub fn can_cast(&self, jutsu: Jutsu) -> Result<(), CastError> {
        if self.cooldown_ticks > 0 {
            return Err(CastError::Cooldown(self.cooldown_ticks));
        }
        let need = jutsu.cost(self.ninjutsu);
        if self.chakra < need {
            return Err(CastError::Chakra {
                have: self.chakra,
                need,
            });
        }
        Ok(())
    }

    pub fn commit_cast(&mut self, jutsu: Jutsu) -> Result<(), CastError> {
        self.can_cast(jutsu)?;
        self.chakra -= jutsu.cost(self.ninjutsu);
        self.cooldown_ticks = jutsu.cooldown();
        self.charging = false;
        self.selected = jutsu;
        Ok(())
    }

    pub fn next_jutsu(&mut self) {
        let index = Jutsu::ALL
            .iter()
            .position(|j| *j == self.selected)
            .unwrap_or(0);
        self.selected = Jutsu::ALL[(index + 1) % Jutsu::ALL.len()];
    }

    /// Temporary training route while original quests and combat XP are unported.
    pub fn train(&mut self) -> bool {
        if self.training_ticks > 0 || self.chakra < 5 {
            return false;
        }
        self.chakra -= 5;
        self.training_ticks = 200;
        self.experience = self.experience.saturating_add(1);
        true
    }

    pub fn spend_ninjutsu(&mut self, points: u32) -> bool {
        if points == 0 || points > self.skill_points || points > 1000 {
            return false;
        }
        self.skill_points -= points;
        self.ninjutsu = self.ninjutsu.saturating_add(points).min(1_000_000);
        true
    }
}

pub fn look(yaw: f32, pitch: f32) -> [f64; 3] {
    let yaw = f64::from(yaw).to_radians();
    let pitch = f64::from(pitch).to_radians();
    [
        -yaw.sin() * pitch.cos(),
        -pitch.sin(),
        yaw.cos() * pitch.cos(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_start_and_integer_scaling_boundaries() {
        let p = Character::default();
        assert_eq!((p.level, p.chakra, p.ninjutsu), (0, 25, 25));
        assert_eq!(Jutsu::Fireball.cost(599), 10);
        assert_eq!(Jutsu::Fireball.cost(600), 11);
        assert_eq!(Jutsu::Rasengan.cost(300), 26);
        assert_eq!(Jutsu::Chidori.cost(600), 101);
        assert_eq!(Jutsu::Chidori.damage(600), 35);
    }

    #[test]
    fn passive_regeneration_matches_immediate_then_hundred_tick_cadence() {
        let mut p = Character {
            chakra: 20,
            ..Character::default()
        };
        p.tick();
        assert_eq!(p.chakra, 21);
        for _ in 0..99 {
            p.tick();
        }
        assert_eq!(p.chakra, 21);
        p.tick();
        assert_eq!(p.chakra, 22);
        for _ in 0..1000 {
            p.tick();
        }
        assert_eq!(p.chakra, p.ninjutsu);
    }

    #[test]
    fn rewards_cross_original_five_hundred_level_bracket() {
        let mut p = Character {
            level: 499,
            experience: 125,
            ..Character::default()
        };
        assert!(p.tick());
        assert_eq!(
            (p.level, p.experience, p.skill_points, p.jutsu_points),
            (500, 0, 1, 7)
        );
    }

    #[test]
    fn accumulated_experience_does_not_skip_multiple_levels_in_one_tick() {
        let mut p = Character {
            experience: 100,
            ..Character::default()
        };
        assert!(p.tick());
        assert_eq!((p.level, p.experience), (1, 99));
    }

    #[test]
    fn rejected_cast_does_not_mutate_character() {
        let mut p = Character::default();
        let before = p.clone();
        assert_eq!(
            p.commit_cast(Jutsu::Chidori),
            Err(CastError::Chakra {
                have: 25,
                need: 100
            })
        );
        assert_eq!(p, before);
        p.commit_cast(Jutsu::Fireball).unwrap();
        let before = p.clone();
        assert!(matches!(
            p.commit_cast(Jutsu::Fireball),
            Err(CastError::Cooldown(_))
        ));
        assert_eq!(p, before);
    }

    #[test]
    fn save_restores_progress_but_resets_transient_input_and_timers() {
        let p = Character {
            charging: true,
            cooldown_ticks: 12,
            level: 17,
            skill_points: 3,
            ..Character::default()
        };
        let saved = serde_json::to_string(&p).unwrap();
        let restored: Character = serde_json::from_str(&saved).unwrap();
        assert_eq!((restored.level, restored.skill_points), (17, 3));
        assert!(!restored.charging);
        assert_eq!(restored.cooldown_ticks, 0);
    }

    #[test]
    fn skill_spending_and_training_cannot_underflow_or_ignore_cooldown() {
        let mut p = Character::default();
        assert!(!p.spend_ninjutsu(1));
        assert!(p.train());
        assert!(!p.train());
        p.tick();
        assert_eq!(p.skill_points, 1);
        assert!(p.spend_ninjutsu(1));
        assert_eq!((p.skill_points, p.ninjutsu), (0, 26));
    }

    #[test]
    fn view_direction_uses_minecraft_yaw_and_pitch() {
        let south = look(0.0, 0.0);
        assert!((south[2] - 1.0).abs() < 1e-12);
        let west = look(90.0, 0.0);
        assert!((west[0] + 1.0).abs() < 1e-12);
        assert!((look(0.0, -90.0)[1] - 1.0).abs() < 1e-12);
    }
}
