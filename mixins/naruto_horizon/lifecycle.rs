fn hook(game: &mut crate::game::Game<'_>, phase: crate::game::native_lifecycle::WorldPhase) {
    let opened = matches!(
        phase,
        crate::game::native_lifecycle::WorldPhase::WorldOpenReady
    );
    let closed = matches!(
        phase,
        crate::game::native_lifecycle::WorldPhase::WorldCloseFinal
            | crate::game::native_lifecycle::WorldPhase::WorldCloseFailed
    );
    crate::game::naruto_horizon::lifecycle(game, opened, closed);
}
