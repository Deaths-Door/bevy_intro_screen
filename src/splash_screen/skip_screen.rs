use super::{ShowSplashScreen, SplashDuration, SplashPreferences};
use bevy::{prelude::*, state::state::FreelyMutableState};

impl<S, D, U> SplashPreferences<S, D, U>
where
    S: States + FreelyMutableState + Clone,
    D: SplashDuration,
    U: ShowSplashScreen,
{
    pub(super) fn add_skip_screen_subsystem(&self, app: &mut App) {
        app.add_systems(Update, Self::skip_screen.run_if(super::state::is_running));
    }

    fn skip_screen(
        input: Res<ButtonInput<KeyCode>>,
        mut state: ResMut<NextState<S>>,
        settings: Res<SplashPreferences<S, D, U>>,
    ) {
        if input.any_just_pressed([KeyCode::Escape, KeyCode::Space, KeyCode::Enter]) {
            state.set(settings.transition_to.clone());
        }
    }
}
