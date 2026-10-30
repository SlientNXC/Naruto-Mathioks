# Original Mathioks mod and requirements

Verified 30 October 2026. The user's downloaded launcher is Horizon Launcher 0.2.29 for Windows x64.

## Original release

- Creator: Mathioks.
- Release filename: `Naruto-Anime-Mod_V0.9.17_Beta_1.7.10-1.0.jar`.
- Original author's post: https://www.minecraftforum.net/forums/mapping-and-modding-java-edition/minecraft-mods/wip-mods/2341656-naruto-anime-mod
- Author-linked download: https://www.mediafire.com/file/i4x3ych7sd1vur1/Naruto-Anime-Mod_V0.9.17_Beta_1.7.10-1.0.jar
- Downloaded jar SHA-256: `b8a65d54d1f435d7b7acd703bc179dd851b9a099b4e0bfac8cdb842c7dace985`.
- The jar contains 2,245 `.class` files and 1,586 PNG files. Its metadata credits Mathioks, identifies mod ID `AM`, targets Minecraft `1.7.10`, and lists an empty extra-dependencies array. The internal metadata version is `1.0`; the author-distributed filename carries the `V0.9.17 Beta` release label.
- This is the original author-linked release, rather than Naruto Anime Mod Plus, Naruto Destination or Final History.

## Requirements to run the original Java release

1. An owned copy of Minecraft Java Edition and a separate Minecraft **1.7.10** instance.
2. **Minecraft Forge for 1.7.10**. Forge's official recommended/latest legacy build is **10.13.4.1614**: https://files.minecraftforge.net/net/minecraftforge/forge/index_1.7.10.html
3. **Java 8** for the legacy Minecraft instance. Prism's official documentation selects Java 8 for pre-1.17 Minecraft: https://prismlauncher.org/wiki/getting-started/installing-java/
4. The original mod jar in that instance's `mods/` folder.

No additional mod is declared in the original jar's metadata. This is a static inspection result; an original Forge-instance runtime test has not been performed in this session. Mods shown in Mathioks' showcase videos are not automatically dependencies.

## Why this does not run as-is in Horizon's native game

Horizon is a Rust game engine. It can carry over some Java mods' data and assets but does not execute Java mod code. Clans, story logic, jutsu, custom entities, player stats and UI must be reimplemented natively. Source:

- https://playhorizon.dev/docs/modpacks
- https://playhorizon.dev/docs/mods
- https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md

Horizon Launcher 0.2.29 also has Java-instance functions, but its loader setup advertises Fabric and Quilt rather than the original Forge 1.7.10 route. The user explicitly chose a native Horizon rebuild.

## Native rebuild requirements and install-link format

- Horizon Launcher with native mod/GitHub import support; 0.2.29 was inspected.
- A working Horizon resource preparation from the player's own Minecraft Java Edition, with ownership verified by the launcher.
- The portable Rust/MinGW tools Horizon downloads for native builds. Forge and Java 8 are not runtime dependencies of the native rewrite.
- This prototype pins Horizon `34118b285fee43f88d39824e78069902b3d508ed` and horizon-mixins `3f99a5c60dfa311cc365524c44ae2fd799bfa1de`.
- Native source can be delivered as `.hnmod` or a public GitHub repository containing a pinned `horizon-mixin.toml` bundle. Horizon supports **Mods → Add from GitHub**, followed by the native build step. A first build may take 20–30 minutes.
- Original textures/models/audio are not redistributed here. A local asset conversion workflow and the relevant rights must be resolved before matching the full original presentation.

The native rebuild included here is unfinished. Read `source/COVERAGE.md`; the package does not contain the original mod's complete gameplay.
