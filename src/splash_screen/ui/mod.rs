#[cfg(feature = "assets")]
mod assets;
mod failure;

#[cfg(feature = "assets")]
pub use assets::*;
pub use failure::*;

use super::{IntroDuration, IntroPreferences};
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

/// This trait provides a generic interface for defining custom splash screen content.
pub trait ShowIntroScreen: Send + Sync + 'static {
    /// Configures the splash screen UI.
    fn configure_ui<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States,
        D: IntroDuration,
        U: ShowIntroScreen;
}

#[cfg(all(feature= "assets",any(feature="bevy_ui")))]
/// A utility trait for managing assets related to an intro screen.
pub trait IntroScreenAssets : AssetCollection {
    /// Returns an optional handle to the background image.
    ///
    /// If no background image is available, returns `None`.
    fn background(&self) -> Option<&Handle<Image>>;

    /// Returns a handle to the icon image.
    fn icon(&self) -> &Handle<Image>;

    /// Returns the label text for the intro screen.
    fn label(&self) -> String;
}