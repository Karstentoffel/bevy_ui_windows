use bevy::prelude::*;
use crate::prelude::*;

pub struct BevyUiWindows;
impl Plugin for BevyUiWindows {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CursorResourcePlugin,
                InteractionButtonSystemsPlugin,
                ContextMenuSystemsPlugin,
            ))
        ;
    }
}