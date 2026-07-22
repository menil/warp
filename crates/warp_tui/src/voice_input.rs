//! TUI presentation wrapper around the shared voice-input lifecycle.
//!
//! The recorder and transcriber remain app singletons. This model emits TUI
//! redraw events while delegating state transitions to the lifecycle shared
//! with the GUI surfaces.

use warp::tui_export::VoiceInputLifecycle;
pub(crate) use warp::tui_export::VoiceInputLifecycleState as TuiVoiceInputState;
use warpui_core::{Entity, ModelContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TuiVoiceInputEvent {
    StateChanged(TuiVoiceInputState),
    Completed(String),
    Failed(String),
}

/// Entity-facing projection of the shared lifecycle for TUI redraws.
pub(crate) struct TuiVoiceInputModel {
    lifecycle: VoiceInputLifecycle,
}

impl Entity for TuiVoiceInputModel {
    type Event = TuiVoiceInputEvent;
}

impl TuiVoiceInputModel {
    pub(crate) fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            lifecycle: VoiceInputLifecycle::default(),
        }
    }

    pub(crate) fn state(&self) -> TuiVoiceInputState {
        self.lifecycle.state()
    }

    pub(crate) fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }

    pub(crate) fn start(&mut self, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.start() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn stop(&mut self, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.begin_transcribing() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn cancel(&mut self, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.cancel() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn complete(&mut self, text: String, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.complete() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::Completed(text));
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn fail(&mut self, hint: String, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.fail() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::Failed(hint));
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }
}

#[cfg(test)]
#[path = "voice_input_tests.rs"]
mod tests;
