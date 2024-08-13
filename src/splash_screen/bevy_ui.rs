use bevy::prelude::*;
use crate::prelude::{IntroScreenAssets,IntroState,ShowIntroScreen};
use std::marker::PhantomData;

/// Represents an intro screen.
pub struct BevyIntroScreen<T>(PhantomData<T>) where T : IntroScreenAssets;

impl<T> Default for BevyIntroScreen<T> where T : IntroScreenAssets {
    fn default() -> Self {
        Self(PhantomData::<T>)
    }
}

impl<T> ShowIntroScreen for BevyIntroScreen<T>
where T : IntroScreenAssets
{
    fn configure_ui<S, D, U>(
        &self,
        app: &mut bevy::prelude::App,
        _: &super::IntroPreferences<S, D, U>,
    ) where
        S: bevy::prelude::States,
        D: super::IntroDuration,
        U: ShowIntroScreen,
    {
        app.add_systems(OnEnter(IntroState::Running),spawn_ui::<T>)
            .add_systems(OnExit(IntroState::Running),despawn_ui::<T>);
    }
}

/// A marker component for entities representing the background of a UI element.
#[derive(Component, Clone)]
pub struct BevyBackgroundMarker;

/// A marker component for entities representing icons within a UI.
#[derive(Component, Clone)]
pub struct BevyIconMarker;

/// A marker component for entities representing text labels in a UI.
#[derive(Component, Clone)]
pub struct BevyLabelMarker;

/// An internal marker component for UI-related entities.
#[derive(Component, Clone)]
struct BevyUiMarker;


// the zindex the label an dicon is spawned on so that more entities can be spawned 'below' and 'above' them
pub const BEVY_INTRO_SCREEN_CONTENT_ZINDEX : ZIndex = ZIndex::Local(10);

fn despawn_ui<T>(mut commands : Commands,query : Query<Entity,With<BevyUiMarker>>) where T : IntroScreenAssets {
    commands.entity(query.single()).despawn_recursive();
}

fn spawn_ui<T>(mut commands : Commands,assets : Res<T>) where T : IntroScreenAssets {
    commands.spawn((
        BevyUiMarker,
        NodeBundle {
            style : Style {
                height : Val::Percent(100.0),
                width : Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        }
    )).with_children(|layout| {
        if let Some(background) = assets.background() {
            layout.spawn(BevyBackgroundMarker::bundle(background))
                .with_children(|builder| main_content::<T>(builder,assets));
        } else {
            main_content::<T>(layout,assets);
        }
    });
}

impl BevyBackgroundMarker {
    fn bundle(background : &Handle<Image>) -> impl Bundle {
        (
            BevyBackgroundMarker,
            ImageBundle {
                image : UiImage {
                    texture : background.clone(),
                    ..Default::default()
                },
                style : Style {
                    height : Val::Percent(100.0),
                    width : Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    }
}

impl BevyIconMarker {
    fn bundle(icon : &Handle<Image>) -> impl Bundle {
        (
            Self,
            ImageBundle {
                image : UiImage {
                    texture : icon.clone(),
                    ..Default::default()
                },
                style : Style {
                    max_height : Val::Px(200.0),
                    max_width : Val::Px(200.0),
                    ..Default::default()
                },
                z_index : BEVY_INTRO_SCREEN_CONTENT_ZINDEX,
                ..Default::default()
            }
        )
    }
}

impl BevyLabelMarker {
    fn bundle(label : &str) -> impl Bundle {
        (
            Self,
            TextBundle {
                text : Text::from_section(
                    label,
                    TextStyle {
                        font_size : 36.0,
                        ..Default::default()
                    }
                ),
                z_index : BEVY_INTRO_SCREEN_CONTENT_ZINDEX,
                ..Default::default()
            }
        )
    }
}

fn main_content<T>(builder : &mut ChildBuilder<'_>,assets : Res<T>) where T: IntroScreenAssets {
    builder.spawn((
        NodeBundle {
            style :  Style {
                height : Val::Percent(100.0),
                width : Val::Percent(100.0),

                // https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_flexible_box_layout/Aligning_items_in_a_flex_container
                // display: flex;
                // align-items: center;
                // justify-content: center;
                display : Display::Flex,
                align_items : AlignItems::Center,
                justify_content : JustifyContent::Center,
                flex_direction : FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        }
    )).with_children(|layout| {
        layout.spawn(BevyIconMarker::bundle(assets.icon()));
        layout.spawn(
            NodeBundle {
                style : Style {
                    margin : UiRect::bottom(Val::Px(25.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
        layout.spawn(BevyLabelMarker::bundle(&assets.label()));
    });
}