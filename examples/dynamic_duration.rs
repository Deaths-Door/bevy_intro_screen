
use bevy::prelude::*;
use bevy_intro_screen::prelude::{*,egui::EguiIntroScreen};
use bevy_egui::EguiContexts;
use std::time::Duration;
use strum::{IntoEnumIterator,EnumIter};
use bevy::dev_tools::states::log_transitions;
fn main() {    
    App::new()
        .add_plugins(AppPlugin)
        .run();
}

const APP_NAME: &'static str = "My game";

const LABEL: &'static str = "DYNAMIC_DURATION SCREEN";

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
            .duration(DownloadAllAssets)
            .ui(GameIntroScreen)
            .build();

        let intro_plugin = IntroScreenPlugin::builder()
            .preferences(preferences)
            .failure_manager(OnFailureContinue)
            .build();

        app.add_plugins(intro_plugin);

        // Change of states is visible from the logs produced by these
        app.add_systems(Update, (log_transitions::<DownloadState>,log_transitions::<DynamicDurationState>));
    }
}

#[derive(Resource,Clone)]
pub struct DownloadAllAssets;

#[derive(States,EnumIter,Clone ,PartialEq , Eq , Hash , Debug,Copy,Default)]
pub enum DownloadState {
    #[default]
    Models,
    Audio,
    UserSettings
}

impl IntroDuration for DownloadAllAssets {
    fn configure_duration<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
        where
            S: States + bevy::state::state::FreelyMutableState,
            D: IntroDuration,
            U: ShowIntroScreen {
        app.init_state::<DownloadState>();

        let generic = GenericDynamicDuration::new(Duration::from_secs(60));
        generic.configure_duration(app,preferences);

        app.add_systems(OnEnter(IntroState::Running),download_assets);
    }
}

fn download_assets(mut intro_next_state : ResMut<NextState<DynamicDurationState>>,mut stage_next_state : ResMut<NextState<DownloadState>>) {
    for (index,_) in DownloadState::iter().enumerate() {
        // Varied duration required for completion
        std::thread::sleep(Duration::from_secs(index as u64));

        if let Some(state) = DownloadState::iter().nth(index + 1usize) {
            stage_next_state.set(state);
        }
    }

    intro_next_state.set(DynamicDurationState::Completed);
}

// ---- Same as egui example ----
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