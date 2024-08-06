use bevy::{prelude::*, state::state::FreelyMutableState, time::Timer};
use std::time::Duration;

use crate::splash_screen::{ShowIntroScreen, IntroPreferences};

use super::IntroDuration;

/// Represents a fixed-duration countdown for the splash screen that transitions to a specified state upon completion.
#[derive(Clone, Debug, Resource)]
pub struct FixedDuration<S>
where
    S: States,
{
    timer: Timer,
    /// S to transition to after the countdown is complete.
    next_state: S,
}

impl<S> FixedDuration<S>
where
    S: States,
{
    /// Creates a new `FixedDuration` with the specified duration and next state.
    pub fn new_with_duration(duration: Duration, next_state: S) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            next_state,
        }
    }

    /// Creates a new `FixedDuration` with the default duration (1.5 seconds) and next state.
    pub fn new(next_state: S) -> Self {
        const SPLASH_DEFAULT_DURATION: Duration = Duration::from_millis(1500);
        Self::new_with_duration(SPLASH_DEFAULT_DURATION, next_state)
    }
}

impl<S> IntroDuration for FixedDuration<S>
where
    S: States + FreelyMutableState,
{
    fn configure_duration<_State, D, U>(&self, app: &mut App, _: &IntroPreferences<_State, D, U>)
    where
        _State: States + FreelyMutableState,
        D: IntroDuration,
        U: ShowIntroScreen,
    {
        app.insert_resource(self.clone())
            .add_systems(Update, Self::only_run_if(Self::tick_time));
    }
}

impl<S> FixedDuration<S>
where
    S: States + FreelyMutableState,
{
    /// Updates the internal timer based on the delta time.
    ///
    /// This system is only run when the splash screen is active.
    /// If the timer reaches zero, transitions to the specified next state.
    fn tick_time(
        mut next_state: ResMut<NextState<S>>,
        mut countdown: ResMut<Self>,
        time: Res<Time>,
    ) {
        countdown.timer.tick(time.delta());
        if countdown.timer.finished() {
            next_state.set(countdown.next_state.clone());
        }
    }
}
