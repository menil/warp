use warpui_core::App;

use super::{TuiVoiceInputModel, TuiVoiceInputState};

#[test]
fn production_model_rejects_overlapping_sessions() {
    App::test((), |mut app| async move {
        let model = app.add_model(TuiVoiceInputModel::new);
        model.update(&mut app, |voice, ctx| {
            assert!(!voice.complete("early".to_owned(), ctx));
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
fn completion_requires_transcribing_state() {
    App::test((), |mut app| async move {
        let model = app.add_model(TuiVoiceInputModel::new);
        model.update(&mut app, |voice, ctx| {
            assert!(voice.start(ctx));
            assert!(!voice.complete("early".to_owned(), ctx));
            assert!(voice.stop(ctx));
            assert!(voice.complete("hello".to_owned(), ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Idle);
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
            assert!(voice.stop(ctx));
            assert!(voice.fail("failed".to_owned(), ctx));
            assert_eq!(voice.state(), TuiVoiceInputState::Idle);
        });
    });
}
