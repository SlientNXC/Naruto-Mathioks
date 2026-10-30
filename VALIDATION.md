# Validation status — 30 October 2026

**Version 0.1.1 is an in-game-tested development prototype. Most of the original Mathioks mod remains unported. No original-mod runtime parity comparison has been performed.**

## Build and import

| Check | Result |
| --- | --- |
| Original author's jar provenance and metadata | Verified; see ORIGINAL-MOD.md |
| Standalone chakra/progression/jutsu mechanics tests | 8 passed, 0 failed |
| Standalone Rust formatting and Clippy with warnings denied | Passed |
| Pinned Horizon AST integration | Passed: 7 hooks, 2 support modules |
| Updated fast-profile game/control binary build | Passed: 0.1.1 game rebuilt in 5m53s |
| Source redistribution audit | Passed: 22 files, 0 failures, 0 warnings |
| Native 0.1.1 package | Passed the pinned Horizon `Bundle::read` validator; 11 source entries |
| GitHub source-link import | Passed on Horizon Launcher 0.2.29 in a separate launcher folder; installed 0.1.1 |

Build target: Horizon `34118b285fee43f88d39824e78069902b3d508ed`, horizon-mixins `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`, Rust 1.99.1, Windows x64 GNU. The local fast build omits optional audio/controller features. Normal launcher builds use the launcher's supported feature selection.

The bare repository link resolved to native-source commit `d0e480ca7909fc8a48607dafe9010a1683c3a682`. All 10 installed Rust/TOML files matched the local source byte for byte. The launcher-generated bundle SHA-256 was `fa53a23dbd332346dd6e4b85b7657dbf60053f73d628bf861c45f945c6d032ea`. The separately packaged deterministic local `.hnmod` was `beac7d33c0244a9f3177d890789213adf56226f91217ab31b7f612df30c2664e` (different archive packaging, same native source). Later documentation-only commits do not change this source.

## Running-game observations

Tests used the actual modded Horizon executable with its documented offscreen renderer and local control API, a new isolated Creative world, and privately copied prepared Minecraft 26.3 resources. Screenshots were captured and visually inspected. No existing player world, account file or installed game setting was modified.

The first test of 0.1.0 found that the mod mistook an integrated server for a dedicated server. That version refused all character commands. Version 0.1.1 checks Horizon's explicit execution role and passed the B-key regression test.

| Action | Observed result in 0.1.1 |
| --- | --- |
| B: create character | Level 0, 25/25 chakra, Ninjutsu 25, SP 0, JP 0 |
| Train once, then advance one server tick | Chakra 21, level 1, XP 0, SP 1, JP 1; includes the eligible passive recovery |
| Spend one skill point | Ninjutsu increased to 26; SP became 0 |
| Creative test setup | `/naruto demo` set Ninjutsu/chakra to 600 for combat checks |
| Rasengan against a 100-HP iron golem | Health 100 → 90; chakra 600 → 573; knockback moved the mob away |
| Cast again during cooldown | Rejected with a wait message; no additional chakra spent |
| Chidori without an in-range target | Rejected; chakra remained 573 |
| Chidori against the same golem, returned into range | Health 90 → 55; chakra 573 → 472 |
| Fireball after resetting the Creative setup | Visible native fireball projectile; chakra 600 → 589 |
| Charge for 10 server ticks | Chakra 589 → 590 |
| N and R keys | Jutsu selection changed; charging stopped |
| Close and reopen the executable | Level 1, Ninjutsu 600, SP 0, JP 1 and Rasengan selection restored; chakra 590 restored and naturally regenerated to 591 |
| J key after reload | Rasengan cast through the normal input path; test mob health 55 → 45 |
| Switch to a new world, then return | New world started at level 0 with 25/25 chakra; original world retained level 1, Ninjutsu 600 and its own spent-chakra value |

The three attacks remain partial implementations. These observations do not establish original flight, explosion, dash, paralysis, animation or progression parity. Fireball's original damage scaling is not implemented; its standalone helper formula is not used by the native projectile.

The original Forge mod has not been run as a comparison oracle. Multiplayer, other original content, full normal-launcher account preparation and other hardware configurations remain untested. The launcher's last account summary reported ownership as unverified; no authentication or ownership check was changed. Development testing used prepared resources already available locally through the launcher's normal preparation path.
