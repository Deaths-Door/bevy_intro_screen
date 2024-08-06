use std::marker::PhantomData;

use bevy::prelude::*;
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

impl<T> IntroAssetLoader<T>
where
    T: AssetCollection,
{
    // Creates IntroAssetLoader
    pub const fn new() -> Self {
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
        preferences: &crate::splash_screen::IntroPreferences<S, D, U>,
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

        app.add_systems(OnExit(preferences.run_at.clone()), remove_asset::<T>);
    }
}

fn remove_asset<T>(mut cmds: Commands)
where
    T: AssetCollection,
{
    cmds.remove_resource::<T>()
}
