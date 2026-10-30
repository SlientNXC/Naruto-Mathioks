fn hook(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch, corners: &mut horizon_client::extras_hud::Corners) {
    crate::game::naruto_horizon::hud(game, batch, corners);
}
