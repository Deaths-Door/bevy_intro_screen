use std::marker::PhantomData;
use bevy_asset_loader::prelude::*;

use crate::splash_screen::IntroState;

use super::ShowIntroScreen;

/// Handles loading assets for the splash screen
pub struct IntroAssetLoader<T>
where
    T: AssetCollection,
{
    _phantom: PhantomData<T>,
}

impl<T> Default for IntroAssetLoader<T>
where
    T: AssetCollection,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T> ShowIntroScreen for IntroAssetLoader<T>
where
    T: AssetCollection,
{
    fn configure_ui<S, D, U>(
        &self,
        app: &mut bevy::prelude::App,
        _: &crate::splash_screen::IntroPreferences<S, D, U>,
    ) where
        S: bevy::prelude::States,
        D: crate::splash_screen::IntroDuration,
        U: ShowIntroScreen,
    {
        app.add_loading_state(
            LoadingState::new(IntroState::Loading)
                .continue_to_state(IntroState::Running)
                .on_failure_continue_to_state(IntroState::Failure)
                .load_collection::<T>(),
        );
    }
}

