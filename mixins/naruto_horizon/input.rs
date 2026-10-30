fn hook(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::naruto_horizon::input(game, key, pressed);
}
