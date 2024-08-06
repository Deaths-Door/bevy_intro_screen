use bevy::prelude::*;
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
        // Use multiple failure stragies together
        .failure_manager(LogFailure.and(OnFailureCloseWindowWithDelay(
            std::time::Duration::from_secs(3),
        )))
        .build();

    App::new()
        .add_plugins((DefaultPlugins, splash_plugin))
        .run();
}

#[derive(Clone)]
pub struct LogFailure;
impl SplashFailureManager for LogFailure {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + bevy::state::state::FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        app.add_systems(schedule, log);
    }
}

fn log(_: Commands) {
    eprintln!("the game has failed!!!1");
}

/// Same ----
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum AppState {
    SplashScreen,
    Menu,
}

#[derive(Clone)]
pub struct GameSplashScreen;

impl ShowSplashScreen for GameSplashScreen {
    fn configure_ui<S, D, U>(&self, _: &mut App, _: &SplashPreferences<S, D, U>)
    where
        S: States,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        // Do nothing
    }
}
