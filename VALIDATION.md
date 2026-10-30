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
| Updated fast-profile game/control binary build | Passed: both executables built successfully in 9m52s |
| Native `.hnmod` package | Passed the pinned Horizon `Bundle::read` validator; 11 entries |
| Built game launch API startup | Passed; the built executable returned its launch API |
| GitHub source-link import in Horizon Launcher | Passed on Launcher 0.2.29 in an isolated test folder |
| Actual in-game verification | Not performed; prepared Minecraft resources/ownership verification needed |
| Comparison with the original mod running in Forge | Not performed |

Build target: Horizon `34118b285fee43f88d39824e78069902b3d508ed`, horizon-mixins `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`, Rust 1.99.1, Windows x64 GNU. The local fast build omits optional audio/controller features; players' normal launcher builds use the launcher's supported feature selection.

No player worlds or existing game settings have been modified. Build/export/test files use isolated workspace folders. Original jars, inspected Java and assets are excluded from this repository.

The bare repository link resolved to source commit `ae1cc4b2250cdad4f85e202664b737b27ef00c29` and installed `naruto_horizon 0.1.0`. Every imported Rust/TOML source file matched the local validated source byte for byte. The launcher's generated bundle SHA-256 was `c01aac7f9441026ca3a4eb196f713854e5296dda30c3b4d22d741ba91b0aea31`; the separately packaged local `.hnmod` was `e31ea7e56ce15c7f71af303b6fa0ae3147401131ba3b88c4fc8362a2d1e0b8c7` (different archive packaging, same native source).

The final source redistribution audit passed with 0 failures and 0 warnings. Later documentation-only commits do not change the native bundle source.

The launcher still reported the signed-in Minecraft account's ownership as unverified during the final read-only check. Authentication checks have not been bypassed. In-game testing, screenshots, and original-mod parity checks remain pending.
