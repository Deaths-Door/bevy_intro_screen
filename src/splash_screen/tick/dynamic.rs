use std::time::Duration;

use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::splash_screen::{ShowSplashScreen, SplashPreferences, SplashState};

use super::{FixedDuration, SplashDuration};

/// Represents the possible states of a dynamic duration.
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]

pub enum DynamicDurationState {
    /// Indicates a failure during the dynamic duration calculation.
    Failure,
    /// Indicates that the dynamic duration has completed.
    #[default]
    Completed,
    /// Indicates that the dynamic duration is currently running.
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
    pub fn new(max_duration: impl Into<Duration>) -> Self {
        let max_duration =
            FixedDuration::new_with_duration(max_duration.into(), DynamicDurationState::Completed);

        Self { max_duration }
    }
}

impl AsRef<FixedDuration<DynamicDurationState>> for GenericDynamicDuration {
    fn as_ref(&self) -> &FixedDuration<DynamicDurationState> {
        &self.max_duration
    }
}

impl SplashDuration for GenericDynamicDuration {
    fn configure_duration<S, D, U>(&self, app: &mut App, preferences: &SplashPreferences<S, D, U>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        app.init_state::<DynamicDurationState>();

        app.add_systems(
            Update,
            Self::only_run_if(
                finish_splash::<S, Self, U>.run_if(in_state(DynamicDurationState::Completed)),
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
    preferences: Res<SplashPreferences<S, D, U>>,
) where
    S: bevy::prelude::States + FreelyMutableState + Clone,
    D: SplashDuration,
    U: ShowSplashScreen,
{
    next_state.set(preferences.transition_to.clone())
}

fn change_to_failure(mut next_state: ResMut<NextState<SplashState>>) {
    next_state.set(SplashState::Failure)
}
