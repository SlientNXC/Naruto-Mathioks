# Changes

## 0.1.1 — development prototype

- Fix character creation being refused in ordinary single-player worlds. Horizon sets a legacy dedicated-options field on its integrated server too; the mod now checks the explicit execution role.
- Add a control-API B-key regression check and document the tested in-game prototype behavior.
- Verify attacks, chakra costs, cooldowns, charging, full-process save/reload and separate-world character isolation in the actual native game.

## 0.1.0 — initial development prototype

- Native chakra/progression core, HUD, per-world character saves and three partial jutsu.
- Compiled and passed bundle/import checks, but had not passed an in-game gameplay test. The first gameplay test found the session-classification bug fixed above.

The full original Mathioks mod remains unported; see COVERAGE.md.
