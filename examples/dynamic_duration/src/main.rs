
use std::time::Duration;
use rand::Rng;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_intro_screen::prelude::*;

fn main() {
    let transition_to = AppState::Menu;
    let preferences = SplashPreferences::builder()
        .run_at(AppState::SplashScreen)
        .transition_to(transition_to)
        .skip_on_input(true)
        .duration(FixedDuration::new(transition_to))
        .ui(GameSplashScreen)
        .build();

    let splash_plugin = SplashScreenPlugin::builder()
        .preferences(preferences)
        .failure_manager(OnFailureCloseWindow)
        .build();

    App::new()
        .add_plugins((DefaultPlugins,splash_plugin))
        .run();
}

#[derive(Resource,Clone)]
pub struct DownloadAllAssets;

impl SplashDuration for DownloadAllAssets {
    fn configure_duration<S, D, U>(&self, app: &mut App, _: &SplashPreferences<S, D, U>)
        where
            S: States + bevy::state::state::FreelyMutableState,
            D: SplashDuration,
            U: ShowSplashScreen {
        let generic = GenericDynamicDuration::new(Duration::from_secs(30));

        app.add_system(Startup,download_assets);
    }
}

fn download_assets(next_state : Res<NextState<DynamicDurationState>>) {
    AsyncComputeTaskPool::get().spawn(||{
        let mut rng = rand::thread_rng();

        // Varied duration required for completion
        std::thread::sleep(Duration::from_secs(rng.gen_range(3..10)));

        let state = match rand::gen_range(0..100) % 4 == 0 {
            true => DynamicDurationState::Completed,
            false => DynamicDurationState::Failure
        };

        next_state.set(state);
    });
}

/// Same ----
#[derive(States,Clone ,PartialEq , Eq , Hash , Debug,Copy)]
pub enum AppState {
    SplashScreen,
    Menu
}

#[derive(Clone)]
pub struct GameSplashScreen;

impl ShowSplashScreen for GameSplashScreen {
    fn configure_ui<S, D, U>(&self, _: &mut App, _: &SplashPreferences<S, D, U>)
        where
            S: States,
            D: SplashDuration,
            U: ShowSplashScreen {
        // Do nothing
    }
}