# Validation status — 30 October 2026

**Development prototype. The original mod has not been fully ported. No in-game gameplay test has passed yet.**

| Check | Result |
| --- | --- |
| Original author's jar provenance and metadata | Verified; details in the accompanying requirements report |
| Standalone chakra/progression/jutsu mechanics tests | 8 passed, 0 failed |
| Standalone Rust formatting | Passed |
| Standalone Clippy with warnings denied | Passed; environment emitted a cache hard-link fallback warning |
| Source redistribution audit | Passed: no failures and no warnings |
| Pinned Horizon AST integration | Passed: 7 hooks, 2 support modules |
| Horizon game source compilation check | Passed; visibility warnings were subsequently corrected |
| Updated fast-profile game/control binary build | In progress at this snapshot |
| Native `.hnmod` ZIP structure | Built and opened successfully; official bundle-reader check pending |
| GitHub source-link import in Horizon Launcher | Pending publication and verification |
| Actual in-game verification | Not performed; prepared Minecraft resources/ownership verification needed |
| Comparison with the original mod running in Forge | Not performed |

Build target: Horizon `34118b285fee43f88d39824e78069902b3d508ed`, horizon-mixins `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`, Rust 1.99.1, Windows x64 GNU. The local fast build omits optional audio/controller features; players' normal launcher builds use the launcher's supported feature selection.

No player worlds or existing game settings have been modified. Build/export/test files use isolated workspace folders. Original jars, inspected Java and assets are excluded from this repository.
