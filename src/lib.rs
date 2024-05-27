mod absolute_button;
mod ui_window;
mod context_menu;

use bevy::prelude::*;

pub struct BevyUiWindows;
impl Plugin for BevyUiWindows {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                bevy_cursor_tools::CursorResourcePlugin,
                absolute_button::InteractionButtonSystemsPlugin,
                context_menu::ContextMenuSystemsPlugin,
            ))
        ;
    }
}