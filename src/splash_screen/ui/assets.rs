use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::splash_screen::SplashState;

use super::ShowSplashScreen;

/// Handles loading assets for the splash screen
pub struct SplashAssetLoader<T>
where
    T: AssetCollection,
{
    _phantom: PhantomData<T>,
}

impl<T> SplashAssetLoader<T>
where
    T: AssetCollection,
{
    // Creates SplashAssetLoader
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T> ShowSplashScreen for SplashAssetLoader<T>
where
    T: AssetCollection,
{
    fn configure_ui<S, D, U>(
        &self,
        app: &mut bevy::prelude::App,
        preferences: &crate::splash_screen::SplashPreferences<S, D, U>,
    ) where
        S: bevy::prelude::States,
        D: crate::splash_screen::SplashDuration,
        U: ShowSplashScreen,
    {
        app.add_loading_state(
            LoadingState::new(SplashState::Loading)
                .continue_to_state(SplashState::Running)
                .on_failure_continue_to_state(SplashState::Failure)
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
