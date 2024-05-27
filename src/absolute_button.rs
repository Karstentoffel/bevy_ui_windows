use bevy::prelude::*;
use bevy::window::PrimaryWindow;
#[derive(Debug, Component, Default, Eq, PartialEq)]
/// This is an addon for The Button Component to allow ui-interaction through other blocking ui elements.
pub enum AbsoluteInteraction {
    #[default]
    None,
    Pressed,
    Hovered,
}
impl AbsoluteInteraction {
    pub fn none(&self) -> bool {
        self == &Self::None
    }
    pub fn pressed(&self) -> bool {
        self == &Self::Pressed
    }
    pub fn hovered(&self) -> bool {
        self == &Self::Hovered
    }
}

pub struct InteractionButtonSystemsPlugin;
impl Plugin for InteractionButtonSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, absolute_interaction)
        ;
    }
}

fn absolute_interaction(
    mut query: Query<(&mut AbsoluteInteraction, &GlobalTransform, &Node), With<Button>>,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(Some(cursor_translation)) = query_windows.get_single().and_then(|w| Ok(w.cursor_position())) {
        for (mut interaction, g_transform, node) in query.iter_mut() {
            if ab_point_intersect(cursor_translation, g_transform.translation().truncate(), node.size()) {
                if mouse_button.pressed(MouseButton::Left) {
                    if *interaction != AbsoluteInteraction::Pressed {
                        *interaction = AbsoluteInteraction::Pressed
                    }
                } else {
                    if *interaction != AbsoluteInteraction::Hovered {
                        *interaction = AbsoluteInteraction::Hovered
                    }
                }
            } else {
                if *interaction != AbsoluteInteraction::None {
                    *interaction = AbsoluteInteraction::None
                }
            }
        }
    }
}

fn ab_point_intersect(point: Vec2, translation: Vec2, scale: Vec2) -> bool {
    point.x >= translation.x - scale.x / 2.0 &&
        point.x <= translation.x + scale.x / 2.0 &&
        point.y >= translation.y - scale.y / 2.0 &&
        point.y <= translation.y + scale.y / 2.0
}