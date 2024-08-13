#[cfg(feature = "egui")]
///
pub mod egui;

#[cfg(feature="bevy_ui")]
///
pub mod bevy_ui;

pub(super) mod skip_screen;
pub(super) mod state;
pub(super) mod tick;
pub(super) mod ui;

use crate::prelude::*;
use bevy::{prelude::*, state::state::FreelyMutableState};
use getset::Getters;
/// The main plugin for Bevy's introduction screen system.
///
/// This plugin manages the splash screen lifecycle, including displaying the screen,
/// handling asset loading, and managing failures.
#[bon::builder]
pub struct IntroScreenPlugin<S, D, U, F>
where
    S: States + FreelyMutableState,
    D: IntroDuration,
    U: ShowIntroScreen,
    F: IntroFailureManager,
{
    preferences: IntroPreferences<S, D, U>,
    failure_manager: F,
}

/// Configuration options for the introduction screen.
///
/// This struct defines the various parameters that control the behavior of the splash screen.
///
/// # Generic Parameters
///
/// * `S`: The state type used by the application.
/// * `D`: The splash screen duration type.
/// * `U`: The type responsible for showing the splash screen
#[derive(Clone, Debug, Resource, Getters)]
#[bon::builder]
pub struct IntroPreferences<S, D, U>
where
    S: States,
    D: IntroDuration,
    U: ShowIntroScreen,
{
    #[getset(get = "pub")]
    pub(in crate::splash_screen) run_at: S,
    #[getset(get = "pub")]
    pub(in crate::splash_screen) transition_to: S,
    #[getset(get = "pub")]
    pub(in crate::splash_screen) skip_on_input: bool,
    #[getset(get = "pub")]
    pub(in crate::splash_screen) duration: D,
    #[getset(get = "pub")]
    pub(in crate::splash_screen) ui: U,
}

impl<S, D, U, F> Plugin for IntroScreenPlugin<S, D, U, F>
where
    S: States + FreelyMutableState,
    D: IntroDuration,
    U: ShowIntroScreen + Clone,
    F: IntroFailureManager + Clone,
{
    fn build(&self, app: &mut App) {
        app.init_state::<state::IntroState>()
            .insert_resource(self.preferences.clone())
            .add_systems(OnEnter(self.preferences.run_at.clone()), change_state)
            .add_systems(
                OnExit(self.preferences.run_at.clone()),
                splash_finish::<S, D, U>,
            );

        self.preferences
            .duration
            .configure_duration(app, &self.preferences);

        self.preferences.ui.configure_ui(app, &self.preferences);

        if self.preferences.skip_on_input {
            self.preferences.add_skip_screen_subsystem(app);
        }

        self.failure_manager
            .manage_failure::<S, D, U>(app, OnEnter(IntroState::Failure))
    }
}

fn change_state(mut next_state: ResMut<NextState<state::IntroState>>) {
    next_state.set(state::IntroState::Loading)
}

fn splash_finish<S: States, D: IntroDuration, U: ShowIntroScreen>(
    mut next_state: ResMut<NextState<state::IntroState>>,
) {
    next_state.set(state::IntroState::Idle);
}
