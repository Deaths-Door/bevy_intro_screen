use bevy::prelude::*;
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
impl IntroFailureManager for LogFailure {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<IntroState>)
    where
        S: States + bevy::state::state::FreelyMutableState,
        D: IntroDuration,
        U: ShowIntroScreen,
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
    IntroScreen,
    Menu,
}

#[derive(Clone)]
pub struct GameIntroScreen;

impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, _: &mut App, _: &IntroPreferences<S, D, U>)
    where
        S: States,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        // Do nothing
    }
}
