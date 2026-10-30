fn hook(game: &mut crate::game::Game<'_>, phase: crate::game::native_lifecycle::ServerTickPhase) {
    let running = matches!(phase, crate::game::native_lifecycle::ServerTickPhase::ServerTickEnd { world_running: true });
    if running { crate::game::naruto_horizon::tick(game); }
}
