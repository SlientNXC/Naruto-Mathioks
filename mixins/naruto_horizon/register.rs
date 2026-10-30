fn hook(
    game: &mut crate::game::Game<'_>,
    commands: &mut Vec<crate::game::native_server::NativeCommand>,
) {
    if game.native_on(crate::game::naruto_horizon::ID) {
        crate::game::naruto_horizon::register(commands);
    }
}
