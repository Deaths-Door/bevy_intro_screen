use bevy::prelude::*;
use bevy_intro_screen::prelude::{*,bevy_ui::BevyIntroScreen};
use std::time::Duration;
use bevy_asset_loader::prelude::AssetCollection;

fn main() {    
    App::new()
        .add_plugins(AppPlugin)
        .run();
}

const APP_NAME: &'static str = "My game";

const LABEL: &'static str = "BEVY_UI INTRO SCREEN";

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

        use bevy::dev_tools::states::log_transitions;
        app.add_systems(Update, (log_transitions::<AppState>,log_transitions::<IntroState>));

        let transition_to = AppState::GameMenu;
        let preferences = IntroPreferences::builder()
            .run_at(AppState::SplashScreen)
            .transition_to(transition_to)
            .skip_on_input(true)
            .duration(FixedDuration::new_with_duration(
                Duration::from_millis(500000000000000),
                transition_to,
            ))
            .ui(GameIntroScreen)
            .build();

        let intro_plugin = IntroScreenPlugin::builder()
            .preferences(preferences)
            .failure_manager(OnFailureContinue)
            .build();

        app.add_plugins(intro_plugin);
    }
}

fn setup(mut commands : Commands) {
    commands.spawn(Camera2dBundle::default());
}
#[derive(AssetCollection, Resource)]
struct GameScreenAssets {
    #[asset(path = "../assets/images/blue_background.png")]
    background: Handle<Image>,
    #[asset(path = "../assets/images/app_logo.png")]
    icon : Handle<Image>,
}

impl IntroScreenAssets for GameScreenAssets {
    fn background(&self) -> Option<&Handle<Image>> {
        None
        //Some(&self.background)
    }

    fn icon(&self) -> &Handle<Image> {
        &self.icon
    }

    fn label(&self) -> String {
        LABEL.to_string()
    }
}

impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        IntroAssetLoader::<GameScreenAssets>::default().configure_ui(app,preferences);
        BevyIntroScreen::<GameScreenAssets>::default().configure_ui(app, preferences);
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
