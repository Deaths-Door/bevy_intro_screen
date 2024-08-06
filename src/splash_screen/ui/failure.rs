use super::ShowSplashScreen;
use crate::splash_screen::{SplashDuration, SplashPreferences, SplashState};
use bevy::{prelude::*, state::state::FreelyMutableState};
use std::{marker::PhantomData, time::Duration};

/// Defines a trait for managing asset loading failures during the splash screen.
pub trait SplashFailureManager: Send + Sync + 'static {
    /// Manages asset loading failures.
    ///
    /// This method is called when the splash screen enters the `Failure` state.
    /// The provided `schedule` should be used to schedule systems or events
    /// to handle the failure
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen;

    /// Combines two `SplashFailureManager` instances into a single one.
    ///
    /// The resulting `And` type will sequentially call `manage_failure` on both
    /// instances when invoked
    fn and<B>(self, other: B) -> And<Self, B>
    where
        B: SplashFailureManager,
        Self: Sized,
    {
        And {
            first: self,
            second: other,
        }
    }
}

/// Creates a new `And` instance combining two `SplashFailureManager` instances.
///
/// This is typically created using the [`SplashFailureManager::and`]
#[derive(Clone)]
pub struct And<A, B>
where
    A: SplashFailureManager,
    B: SplashFailureManager,
{
    first: A,
    second: B,
}

/// Closes the window when a splash screen failure occurs.
#[derive(Clone)]
pub struct OnFailureCloseWindow;

/// Continues to the next state (aka [SplashPreferences::transition_to]) when a splash screen failure occurs.
#[derive(Clone)]
pub struct OnFailureContinue;

/// Closes the window **after a delay** when a splash screen failure occurs.
#[derive(Clone)]
pub struct OnFailureCloseWindowWithDelay(pub Duration);

/// Continues to the next state (aka [SplashPreferences::transition_to]) **after a delay** when a splash screen failure occurs.
#[derive(Clone)]
pub struct OnFailureContinueWithDelay(pub Duration);

// used to create the `*WithDelay`` Structs and so that they have their own timer
#[derive(Resource, Clone)]
struct WithDelay<T>
where
    T: Resource,
{
    timer: Timer,
    owner: PhantomData<T>,
}

impl OnFailureCloseWindow {
    /// Internal system of [OnFailureCloseWindow]
    pub fn system(mut exit: EventWriter<AppExit>) {
        exit.send(AppExit::Success);
    }
}

impl OnFailureContinue {
    /// Internal system of [OnFailureContinue]
    pub fn system<S: States + FreelyMutableState, D: SplashDuration, U: ShowSplashScreen>(
        mut next_state: ResMut<NextState<S>>,
        conf: Res<SplashPreferences<S, D, U>>,
    ) {
        next_state.set(conf.transition_to.clone())
    }
}

impl<T> WithDelay<T>
where
    T: Resource,
{
    fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            owner: PhantomData::<T>,
        }
    }

    pub fn system(mut countdown: ResMut<Self>, time: Res<Time>) {
        countdown.timer.tick(time.delta());
    }

    pub fn delay_finished(countdown: Res<Self>) -> bool {
        countdown.timer.finished()
    }
}

impl<A, B> SplashFailureManager for And<A, B>
where
    A: SplashFailureManager,
    B: SplashFailureManager,
{
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        self.first.manage_failure::<S, D, U>(app, schedule.clone());
        self.second.manage_failure::<S, D, U>(app, schedule);
    }
}

impl SplashFailureManager for OnFailureCloseWindow {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        app.add_systems(schedule, Self::system);
    }
}

impl SplashFailureManager for OnFailureContinue {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        app.add_systems(schedule, Self::system::<S, D, U>);
    }
}

impl<T> SplashFailureManager for WithDelay<T>
where
    T: Resource + Clone,
{
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        app.insert_resource(self.clone())
            .add_systems(schedule, Self::system);
    }
}

// General Macro to create *With Delay
/*
macro_rules! with_delay_impl {
    ($name : ty,$marker : ident,$inner :ty) => {
        #[derive(Resource,Clone)]
        struct $marker;

        impl SplashFailureManager for $name {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        let delay = WithDelay::<$marker>::new(self.0.clone());
        delay.manage_failure::<S,D,U>(app,schedule.clone());

        app.insert_resource(delay).add_systems(schedule,<$inner>::system.run_if(WithDelay::<$marker>::delay_finished));
    }
}
    }
}*/

#[derive(Resource, Clone)]
struct CloseWindowDelayMaker;

impl SplashFailureManager for OnFailureCloseWindowWithDelay {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        let delay = WithDelay::<CloseWindowDelayMaker>::new(self.0.clone());
        delay.manage_failure::<S, D, U>(app, schedule.clone());
        app.insert_resource(delay).add_systems(
            schedule,
            <OnFailureCloseWindow>::system
                .run_if(WithDelay::<CloseWindowDelayMaker>::delay_finished),
        );
    }
}
#[derive(Resource, Clone)]
struct ContinueDelayMaker;

impl SplashFailureManager for OnFailureContinueWithDelay {
    fn manage_failure<S, D, U>(&self, app: &mut App, schedule: OnEnter<SplashState>)
    where
        S: States + FreelyMutableState,
        D: SplashDuration,
        U: ShowSplashScreen,
    {
        let delay = WithDelay::<ContinueDelayMaker>::new(self.0.clone());
        delay.manage_failure::<S, D, U>(app, schedule.clone());
        app.insert_resource(delay).add_systems(
            schedule,
            <OnFailureContinue>::system::<S, D, U>
                .run_if(WithDelay::<ContinueDelayMaker>::delay_finished),
        );
    }
}
