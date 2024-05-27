pub mod absolute_button;
pub use absolute_button::*;
pub mod ui_window;
pub use ui_window::*;
pub mod context_menu;
pub use context_menu::*;

use bevy::prelude::*;

pub struct BevyUiWindows;
impl Plugin for BevyUiWindows {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                bevy_cursor_tools::CursorResourcePlugin,
                InteractionButtonSystemsPlugin,
                ContextMenuSystemsPlugin,
            ))
        ;
    }
}