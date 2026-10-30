# Naruto Horizon — unfinished native rebuild

This is an independent **development prototype** of a small part of Mathioks' original Naruto Anime Mod. It is not the complete mod, not an official Mathioks release, and not a drop-in conversion of the Java jar. Most original content remains unported. Read [COVERAGE.md](COVERAGE.md) before installing.

## What exists

- Native Rust character state, saved separately in each world's `naruto-horizon/characters.json`.
- Chakra HUD and rebindable keys.
- Original starting chakra/capacity, passive regeneration cadence, level thresholds, skill-point rewards and jutsu-point reward brackets.
- Fireball projectile, Rasengan melee strike and Chidori melee prototype. Their chakra-cost formulas follow the original. Several effects and unlock requirements are still missing.
- Temporary training and chakra-charging controls to exercise the prototype.

This version is intended for a new single-player test world. It has no requirement for Forge or a Java runtime during play. Horizon's launcher downloads its portable Rust build tools when building a native mod.

## Install the development prototype

In Horizon Launcher 0.2.29 or later, open Mods → Add from GitHub and paste:

https://github.com/SlientNXC/Naruto-Mathioks

Review the native-code notice and build the native instance. The first build can take 20–30 minutes. This project pins Horizon commit `34118b285fee43f88d39824e78069902b3d508ed` and horizon-mixins commit `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`. See [VALIDATION.md](VALIDATION.md) for whether the import, build and gameplay checks have actually passed. This link delivers the unfinished prototype described here, not the entire original mod.

## Controls

| Key | Action |
| --- | --- |
| B | Create character / show status |
| R | Toggle prototype chakra charging |
| J | Use selected jutsu |
| N | Select next jutsu |

Keys can be rebound in Options → Mods. `/naruto help` lists commands. `/naruto train` spends 5 chakra for 1 experience, with a 10-second cooldown. `/naruto spend 1` assigns a skill point to Ninjutsu, increasing chakra capacity. Rasengan requires at least 25 chakra; Chidori requires at least 100. In a new Creative test world, an operator can use `/naruto demo` to set that character's Ninjutsu and chakra to 600 for testing all three abilities. This change is permanent for that test character. Training access, free selection of the three jutsu, demo setup and charging are provisional rather than original story progression.

## Developer checks

`cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check` check the standalone mechanics. Apply the mixin bundle with the pinned horizon-mixins to a separate export of the pinned Horizon source, then compile that staged game. `node tools/package.cjs` creates a deterministic source-only `.hnmod` in `target/`.

Successful mechanics tests and source compilation do not establish gameplay parity or prove an in-game test. See [VALIDATION.md](VALIDATION.md) for the actual results of this work session.

## Credits

Original Naruto Anime Mod: **Mathioks**. Original author post and download: [Minecraft Forum](https://www.minecraftforum.net/forums/mapping-and-modding-java-edition/minecraft-mods/wip-mods/2341656-naruto-anime-mod). Naruto: Masashi Kishimoto and its respective rights holders. Horizon and horizon-mixins: their respective contributors.

This adaptation contains independently written Rust, created with assistance from OpenAI Codex. It does not include the original mod jar, decompiled Java, Minecraft game files, original textures, models or audio. This repository's existing CC0-1.0 dedication applies to its new code; it grants no rights to the original mod, Minecraft, Naruto, or their assets.
