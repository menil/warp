use warpui_core::App;

use super::{TuiVoiceInputModel, TuiVoiceInputState};

#[test]
fn production_model_rejects_overlapping_sessions() {
    App::test((), |mut app| async move {
        let model = app.add_model(TuiVoiceInputModel::new);
        model.update(&mut app, |voice, ctx| {
            assert!(voice.start(ctx));
            assert!(!voice.start(ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Listening);
            assert!(voice.stop(ctx));
            assert!(!voice.stop(ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Transcribing);
        });
    });
}

#[test]
fn stale_completion_cannot_mutate_a_later_session() {
    App::test((), |mut app| async move {
        let model = app.add_model(TuiVoiceInputModel::new);
        model.update(&mut app, |voice, ctx| {
            assert!(voice.start(ctx));
            let first_generation = voice.generation();
            assert!(voice.stop(ctx));
            assert!(voice.complete(first_generation, "hello".to_owned(), ctx));

            assert!(voice.start(ctx));
            let second_generation = voice.generation();
            assert_ne!(first_generation, second_generation);
            assert!(voice.stop(ctx));
            assert!(!voice.complete(first_generation, "stale".to_owned(), ctx));
            assert!(!voice.fail(first_generation, "stale".to_owned(), ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Transcribing);
        });
    });
}

#[test]
fn cancellation_and_failure_return_to_idle() {
    App::test((), |mut app| async move {
        let model = app.add_model(TuiVoiceInputModel::new);
        model.update(&mut app, |voice, ctx| {
            assert!(voice.start(ctx));
            assert!(voice.stop(ctx));
            assert!(voice.cancel(ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Idle);

            assert!(voice.start(ctx));
            let generation = voice.generation();
            assert!(voice.stop(ctx));
            assert!(voice.fail(generation, "failed".to_owned(), ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Idle);
        });
    });
}
