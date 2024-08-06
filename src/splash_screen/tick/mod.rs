mod dynamic;
mod fixed;

pub use dynamic::*;
pub use fixed::*;

use bevy::{ecs::schedule::SystemConfigs, prelude::*, state::state::FreelyMutableState};

use super::{state::is_running, ShowIntroScreen, IntroPreferences};

/// Defines a trait for configuring the splash screen duration.
pub trait IntroDuration: Resource + Clone {
    /// Configures the splash screen based on the provided `IntroPreferences`.
    fn configure_duration<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States + FreelyMutableState,
        D: IntroDuration,
        U: ShowIntroScreen;

    /// This helper function should be used for systems added using this trait to guarantee
    /// that they are executed only when necessary.
    fn only_run_if<M>(value: impl IntoSystemConfigs<M>) -> SystemConfigs {
        value.run_if(is_running)
    }
}
