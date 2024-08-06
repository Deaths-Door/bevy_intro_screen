
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

#[derive(States,Clone ,PartialEq , Eq , Hash , Debug,Copy)]
pub enum Stage {
    Models,
    Audio,
    Vfx,
    UserSettings,
    Etc
}



impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, app: &mut App, _: &IntroPreferences<S, D, U>)
        where
            S: States,
            D: IntroDuration,
            U: ShowIntroScreen {
        app.init_state::<Stage>();

        // Now you can display some text based on the downloading state
        // app.add_system(Update,show_text.run_if(in_state(..)))
    }
}

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

fn download_assets(intro_next_state : Res<NextState<DynamicDurationState>>,stage_next_state : Res<NextState<Stage>>) {
    AsyncComputeTaskPool::get().spawn(||{
        let mut rng = rand::thread_rng();

        let options = [Models,Audio, Vfx, UserSettings, Etc];
        
        for (index,variant) in &options.enumerate() {
            // Varied duration required for completion
            std::thread::sleep(Duration::from_secs(rng.gen_range(2..5)));

            if index < options.len() {
                stage_next_state.set(options[index + 1]);
            }   
        }

        intro_next_state.set(DynamicDurationState::Completed);
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