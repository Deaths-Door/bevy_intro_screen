use std::borrow::Cow;

use bevy::prelude::*;
use bevy_egui::{
    egui::{
        CentralPanel, Color32, Image as EguiImage, ImageSource, RichText, TopBottomPanel, Vec2,
        Widget,
    },
    EguiContexts, EguiPlugin,
};

use super::{is_running, ShowSplashScreen, SplashFailureManager};

#[derive(Resource, Clone)]
#[bon::builder]
pub struct EguiSplashScreen<'a> {
    background: Option<ImageSource<'a>>,
    icon: ImageSource<'a>,
    label: Cow<'a, str>,
}

// allows for more complex error messages then a simple generic error message
#[derive(Clone)]
pub struct OnFailureShowMessage<T>(pub T)
where
    T: Clone + AsRef<str> + Send + Sync + 'static;

impl<'a> ShowSplashScreen for EguiSplashScreen<'a>
where
    'a: 'static,
{
    fn configure_ui<S, D, U>(
        &self,
        app: &mut bevy::prelude::App,
        _: &super::SplashPreferences<S, D, U>,
    ) where
        S: bevy::prelude::States,
        D: super::SplashDuration,
        U: ShowSplashScreen,
    {
        app.add_plugins(EguiPlugin);

        app.insert_resource(self.clone());

        app.add_systems(Update, splash.run_if(is_running));
    }
}

fn splash(contexts: EguiContexts, assets: Res<EguiSplashScreen<'static>>) {
    CentralPanel::default().show(contexts.ctx(), |ui| {
        if let Some(background) = &assets.background {
            // https://github.com/emilk/egui/discussions/3383#discussioncomment-7373747
            EguiImage::new(background.clone()).paint_at(ui, ui.ctx().screen_rect())
        }

        ui.vertical_centered(|ui| {
            let total_size = ui.available_size();
            let image_size = total_size / 5f32;

            let spacing = image_size.y * 2f32;
            ui.add_space(spacing);

            EguiImage::new(assets.icon.clone())
                .max_size(image_size)
                .ui(ui);

            ui.add_space(image_size.y / 4f32);

            ui.label(RichText::new(assets.label.as_ref()).heading());
        })
    });
}

#[derive(Resource)]
struct FailureMessageResource<T>(T)
where
    T: AsRef<str> + Send + Sync + 'static;

impl<T> SplashFailureManager for OnFailureShowMessage<T>
where
    T: Clone + AsRef<str> + Send + Sync + 'static,
{
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<super::SplashState>)
    where
        S: States + bevy::state::state::FreelyMutableState,
        D: super::SplashDuration,
        U: ShowSplashScreen,
    {
        let on_exit_schedule: OnExit<super::SplashState> = OnExit(schedule.0.clone());
        let message_resource = FailureMessageResource(self.0.clone());

        app.insert_resource(message_resource)
            .add_systems(Update, show_message::<T>.run_if(in_state(schedule.0)))
            .add_systems(on_exit_schedule, clean_up::<FailureMessageResource<T>>);
    }
}

fn show_message<T>(contexts: EguiContexts, res: Res<FailureMessageResource<T>>)
where
    T: AsRef<str> + Send + Sync + 'static,
{
    TopBottomPanel::bottom("splash_screen_snackbar").show(contexts.ctx(), |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.label(
                RichText::new(res.0.as_ref())
                    .monospace()
                    .color(Color32::LIGHT_RED),
            )
        })
    });
}

fn clean_up<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>()
}
