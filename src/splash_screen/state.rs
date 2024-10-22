use bevy::prelude::*;

#[derive(PartialOrd, PartialEq, Copy, Clone, Resource, Default, States, Debug, Hash, Eq)]
/// Represents the different states of a splash screen.
pub enum IntroState {
    /// **For Internal Use Only**
    /// Idle state, before the splash screen starts.
    #[default]
    Idle,
    /// Loading assets for the splash screen.
    Loading,
    /// Intro screen is currently displayed.
    Running,
    /// Error state indicating a failure during splash screen loading.
    Failure,
}

impl IntroState {
    /// Returns `true` if the splash screen is currently running.
    pub const fn is_running(&self) -> bool {
        matches!(&self, IntroState::Running)
    }

    /// Returns `true` if the splash screen is currently loading.
    pub const fn is_loading(&self) -> bool {
        matches!(&self, IntroState::Loading)
    }

    /// Returns `true` if the splash screen is in a failure state.
    pub const fn is_failure(&self) -> bool {
        matches!(&self, IntroState::Failure)
    }
}

/// Checks if the splash screen is currently running.
pub fn is_running(state: Res<State<IntroState>>) -> bool {
    state.is_running()
}

/// Checks if the splash screen is currently loaddng.
pub fn is_loading(state: Res<State<IntroState>>) -> bool {
    state.is_loading()
}

/// Checks if the splash screen has just started running.
pub fn started_running(state: Res<State<IntroState>>) -> bool {
    state.is_changed() && state.is_running()
}

/// Checks if the splash screen is in a failure state.
pub fn is_failure(state: Res<State<IntroState>>) -> bool {
    state.is_failure()
}
