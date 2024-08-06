
use std::time::Duration;
use rand::Rng;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_intro_screen::prelude::*;

fn main() {
    let transition_to = AppState::Menu;
    let preferences = IntroPreferences::builder()
        .run_at(AppState::IntroScreen)
        .transition_to(transition_to)
        .skip_on_input(true)
        .duration(FixedDuration::new(transition_to))
        .ui(GameIntroScreen)
        .build();

    let splash_plugin = IntroScreenPlugin::builder()
        .preferences(preferences)
        .failure_manager(OnFailureCloseWindow)
        .build();

    App::new()
        .add_plugins((DefaultPlugins,splash_plugin))
        .run();
}

#[derive(Resource,Clone)]
pub struct DownloadAllAssets;

impl IntroDuration for DownloadAllAssets {
    fn configure_duration<S, D, U>(&self, app: &mut App, _: &IntroPreferences<S, D, U>)
        where
            S: States + bevy::state::state::FreelyMutableState,
            D: IntroDuration,
            U: ShowIntroScreen {
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
    IntroScreen,
    Menu
}

#[derive(Clone)]
pub struct GameIntroScreen;

impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, _: &mut App, _: &IntroPreferences<S, D, U>)
        where
            S: States,
            D: IntroDuration,
            U: ShowIntroScreen {
        // Do nothing
    }
}