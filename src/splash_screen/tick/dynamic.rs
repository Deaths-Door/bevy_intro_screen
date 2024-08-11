use std::time::Duration;

use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::splash_screen::{ShowIntroScreen, IntroPreferences, IntroState};

use super::{FixedDuration, IntroDuration};

/// Represents the possible states of a dynamic duration.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]

pub enum DynamicDurationState {
    /// Indicates a failure during the dynamic duration calculation.
    Failure,
    /// Indicates that the dynamic duration has completed.
    Completed,
    /// Indicates that the dynamic duration is currently running.
    #[default]
    Running,
}

/// Represents a generic dynamic duration with a maximum duration.
///
/// Provides a basic implementation for dynamic durations by initializing the state and starting a maximum duration countdown.
/// Applications should implement logic to transition from `Running` to either `Failure` or `Completed` based on specific conditions.

#[derive(Clone, Debug, Resource)]
pub struct GenericDynamicDuration {
    /// Maximum allowed duration for the dynamic process.
    max_duration: FixedDuration<DynamicDurationState>,
}

impl GenericDynamicDuration {
    /// Creates a new `GenericDynamicDuration` instance with the specified maximum duration.
    ///
    /// # Arguments
    /// * `max_duration`: The maximum allowed duration. Can be any type that can be converted into a `Duration`.
    ///
    /// # Returns
    /// A new `GenericDynamicDuration` instance.
    pub fn new(max_duration: impl Into<Duration>) -> Self {
        let max_duration =
            FixedDuration::new_with_duration(max_duration.into(), DynamicDurationState::Failure);

        Self { max_duration }
    }
}

impl AsRef<FixedDuration<DynamicDurationState>> for GenericDynamicDuration {
    fn as_ref(&self) -> &FixedDuration<DynamicDurationState> {
        &self.max_duration
    }
}

impl IntroDuration for GenericDynamicDuration {
    fn configure_duration<S, D, U>(&self, app: &mut App, preferences: &IntroPreferences<S, D, U>)
    where
        S: States + FreelyMutableState,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        app.init_state::<DynamicDurationState>();
        app.insert_resource(self.clone());

        app.add_systems(
            Update,
            Self::only_run_if(
                finish_splash::<S, D, U>.run_if(in_state(DynamicDurationState::Completed)),
            ),
        );

        app.add_systems(
            Update,
            Self::only_run_if(change_to_failure.run_if(in_state(DynamicDurationState::Failure))),
        );

        self.max_duration.configure_duration(app, preferences);
    }
}

fn finish_splash<S, D, U>(
    mut next_state: ResMut<NextState<S>>,
    preferences: Res<IntroPreferences<S, D, U>>,
) where
    S: bevy::prelude::States + FreelyMutableState + Clone,
    D: IntroDuration,
    U: ShowIntroScreen,
{
    next_state.set(preferences.transition_to.clone())
}

fn change_to_failure(mut next_state: ResMut<NextState<IntroState>>) {
    next_state.set(IntroState::Failure)
}
