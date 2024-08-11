use bevy::prelude::*;
use bevy_intro_screen::prelude::{*,egui::EguiIntroScreen};
use bevy_egui::EguiContexts;
use std::time::Duration;
fn main() {    
    App::new()
        .add_plugins(AppPlugin)
        .run();
}

const APP_NAME: &'static str = "My game";

const LABEL: &'static str = "EGUI INTRO SCREEN";

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self,app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(APP_NAME),
                ..Default::default()
            }),
            close_when_requested: true,
            ..Default::default()
        }));

        app.add_systems(Startup, setup);

        app.init_state::<AppState>();

        let transition_to = AppState::GameMenu;
        let preferences = IntroPreferences::builder()
            .run_at(AppState::SplashScreen)
            .transition_to(transition_to)
            .skip_on_input(true)
            .duration(FixedDuration::new_with_duration(
                Duration::from_millis(5000),
                transition_to,
            ))
            .ui(GameIntroScreen)
            .build();

        let intro_plugin = IntroScreenPlugin::builder()
            .preferences(preferences)
            // Main Difference
            .failure_manager(LogFailure.and(OnFailureCloseWindow))
            .build();

        app.add_plugins(intro_plugin);

        app.add_systems(OnEnter(IntroState::Running),to_failure);
    }
}

fn to_failure(mut next_state : ResMut<NextState<IntroState>>) {
    next_state.set(IntroState::Failure)
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


// Same as EGUI example

fn setup(contexts: EguiContexts,mut commands : Commands) {
    egui_extras::install_image_loaders(contexts.ctx());
    commands.spawn(Camera2dBundle::default());
}

impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        let egui = EguiIntroScreen::builder()
        .label(LABEL.into())
        .icon(bevy_egui::egui::include_image!(
            "../assets/images/app_logo.png"
        ))
        .background(bevy_egui::egui::include_image!("../assets/images/blue_background.png"))
        .build();

    egui.configure_ui(app, preferences);

 }
}

#[derive(Clone)]
pub struct GameIntroScreen;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    SplashScreen,

    GameMenu,
}