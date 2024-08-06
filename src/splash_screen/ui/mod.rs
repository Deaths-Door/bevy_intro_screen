#[cfg(feature = "assets")]
mod assets;
mod failure;

#[cfg(feature = "assets")]
pub use assets::*;
pub use failure::*;

use super::{SplashDuration, SplashPreferences};
use bevy::prelude::*;

/// This trait provides a generic interface for defining custom splash screen content.
pub trait ShowSplashScreen: Send + Sync + 'static {
    /// Configures the splash screen UI.
    fn configure_ui<S, D, U>(&self, app: &mut App, preferences: &SplashPreferences<S, D, U>)
    where
        S: States,
        D: SplashDuration,
        U: ShowSplashScreen;
}
