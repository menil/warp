use warpui_core::App;

use super::{
    StartListeningError, VoiceInput, VoiceInputLifecycle, VoiceInputLifecycleState,
    VoiceInputState, VoiceInputToggledFrom,
};

#[test]
fn lifecycle_rejects_overlapping_sessions() {
    let mut lifecycle = VoiceInputLifecycle::default();
    let generation = lifecycle.start().expect("idle lifecycle should start");

    assert_eq!(lifecycle.state(), VoiceInputLifecycleState::Listening);
    assert_eq!(lifecycle.start(), None);
    assert!(lifecycle.begin_transcribing(generation));
    assert_eq!(lifecycle.state(), VoiceInputLifecycleState::Transcribing);
    assert_eq!(lifecycle.start(), None);
}

#[test]
fn lifecycle_ignores_stale_completions() {
    let mut lifecycle = VoiceInputLifecycle::default();
    let first_generation = lifecycle.start().expect("first session should start");
    assert!(lifecycle.begin_transcribing(first_generation));
    assert!(lifecycle.complete(first_generation));

    let second_generation = lifecycle.start().expect("second session should start");
    assert_ne!(first_generation, second_generation);
    assert!(lifecycle.begin_transcribing(second_generation));
    assert!(!lifecycle.complete(first_generation));
    assert!(!lifecycle.fail(first_generation));
    assert_eq!(lifecycle.state(), VoiceInputLifecycleState::Transcribing);
}

#[test]
fn lifecycle_cancellation_invalidates_pending_completion() {
    let mut lifecycle = VoiceInputLifecycle::default();
    let generation = lifecycle.start().expect("session should start");
    assert!(lifecycle.begin_transcribing(generation));
    assert!(lifecycle.cancel());

    assert_eq!(lifecycle.state(), VoiceInputLifecycleState::Idle);
    assert!(!lifecycle.complete(generation));
    assert!(!lifecycle.fail(generation));
    assert!(!lifecycle.cancel());
}

#[test]
fn recorder_rejects_a_new_session_while_transcribing() {
    App::test((), |mut app| async move {
        let voice_input = app.add_model(VoiceInput::new);
        voice_input.update(&mut app, |voice_input, ctx| {
            voice_input.state = VoiceInputState::Transcribing;
            assert!(matches!(
                voice_input.start_listening(ctx, VoiceInputToggledFrom::Button),
                Err(StartListeningError::AlreadyRunning)
            ));
        });
    });
}
