use bevy::prelude::*;
use bevy_intro_screen::prelude::*;

fn main() {
    let transition_to = AppState::GameMenu;
    let preferences = IntroPreferences::builder()
        .run_at(AppState::IntroScreen)
        .transition_to(transition_to)
        .skip_on_input(true)
        .duration(FixedDuration::new(transition_to))
        .ui(GameIntroScreen)
        .build();

    let splash_plugin = IntroScreenPlugin::builder()
        .preferences(preferences)
        .failure_manager(OnFailureContinue)
        .build();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(splash_plugin)
        .run()
}


#[derive(Clone)]
pub struct GameIntroScreen;

impl GameIntroScreen {
    pub(super) const LABEL: &'static str = "Custom Egui Example";
}

impl ShowIntroScreen for GameIntroScreen {
    fn configure_ui<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        // so that the image loader is loaded and has enought time to do so
        app.add_systems(Startup, setup);

            
        let egui = EguiIntroScreen::builder()
            .label(Self::LABEL.into())
            .icon(bevy_egui::egui::include_image!(
                "../../assets/images/app_logo.png"
            ))
            .build();

        egui.configure_ui(app, preferences);
    }
}

fn setup(contexts: EguiContexts) {
    egui_extras::install_image_loaders(contexts.ctx());
}


// Same --

#[derive(States,Clone ,PartialEq , Eq , Hash , Debug,Copy)]
pub enum AppState {
    IntroScreen,
    Menu
}