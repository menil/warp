//! TUI presentation wrapper around the shared voice-input lifecycle.
//!
//! The recorder and transcriber remain app singletons. This model emits TUI
//! redraw events while delegating state transitions and stale-result rejection
//! to the lifecycle shared with the GUI surfaces.

use warp::tui_export::VoiceInputLifecycle;
pub(crate) use warp::tui_export::VoiceInputLifecycleState as TuiVoiceInputState;
use warpui_core::{Entity, ModelContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TuiVoiceInputEvent {
    StateChanged(TuiVoiceInputState),
    Completed { generation: u64, text: String },
    Failed { generation: u64, hint: String },
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

    pub(crate) fn generation(&self) -> u64 {
        self.lifecycle.generation()
    }

    pub(crate) fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }

    pub(crate) fn is_transcribing_generation(&self, generation: u64) -> bool {
        self.state() == TuiVoiceInputState::Transcribing && self.generation() == generation
    }

    pub(crate) fn start(&mut self, ctx: &mut ModelContext<Self>) -> bool {
        if self.lifecycle.start().is_none() {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn stop(&mut self, ctx: &mut ModelContext<Self>) -> bool {
        if !self.lifecycle.begin_transcribing(self.generation()) {
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

    pub(crate) fn complete(
        &mut self,
        generation: u64,
        text: String,
        ctx: &mut ModelContext<Self>,
    ) -> bool {
        if !self.lifecycle.complete(generation) {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::Completed { generation, text });
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }

    pub(crate) fn fail(
        &mut self,
        generation: u64,
        hint: String,
        ctx: &mut ModelContext<Self>,
    ) -> bool {
        if !self.lifecycle.fail(generation) {
            return false;
        }
        ctx.emit(TuiVoiceInputEvent::Failed { generation, hint });
        ctx.emit(TuiVoiceInputEvent::StateChanged(self.state()));
        true
    }
}

#[cfg(test)]
#[path = "voice_input_tests.rs"]
mod tests;
