use bevy::prelude::*;
use crate::absolute_button::*;
use bevy::ecs::system::EntityCommands;
use bevy_cursor_tools::Cursor;

#[derive(Component, Debug, Clone)]
pub struct UiWindow;

#[derive(Bundle)]
pub struct UiWindowBundle {
    ui_window: UiWindow,
    node_bundle: NodeBundle,
    absolute_button: AbsoluteInteraction,
    button: Button,
}
impl Default for UiWindowBundle {
    fn default() -> Self {
        Self { ui_window: UiWindow, node_bundle: Self::default_node_bundle(), absolute_button: AbsoluteInteraction::default(), button: Button }
    }
}
impl UiWindowBundle {
    fn default_node_bundle() -> NodeBundle {
        NodeBundle {
            background_color: BackgroundColor(Color::rgba(0.7, 0.7, 0.7, 0.7)),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(200.0),
                height: Val::Px(200.0),
                ..default()
            },
            ..default()
        }
    }
    pub fn with_button_bundle(mut self, node_bundle: NodeBundle) -> Self {
        self.node_bundle = node_bundle;
        self
    }
    pub fn with_style(mut self, style: Style) -> Self {
        self.node_bundle.style = style;
        self
    }
    pub fn with_ui_window(mut self, ui_window: UiWindow) -> Self {
        self.ui_window = ui_window;
        self
    }
    pub fn with_screen_position(mut self, position: Vec2) -> Self {
        if let (Val::Px(width), Val::Px(height)) = &(self.node_bundle.style.width, self.node_bundle.style.height) {
            self.node_bundle.style.left = Val::Px(position.x - width / 2.0);
            self.node_bundle.style.top = Val::Px(position.y - height / 2.0);
        };
        self
    }
}
impl UiWindowBundle {
    pub fn construct<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let window = commands.spawn(self).id();
        let window_bar = UiWindowBar::default().construct(commands).id();
        let window_bar_button = UiButton::new_close_window().construct(commands).id();
        commands.entity(window).push_children(&[window_bar]);
        commands.entity(window_bar).push_children(&[window_bar_button]);
        commands.entity(window)
    }
    pub fn construct_borderless<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let window = commands.spawn(self).id();
        commands.entity(window)
    }
}
#[derive(Debug, Component)]
pub struct UiWindowParent;

#[derive(Component, Default)]
pub struct UiWindowBar;
impl UiWindowBar {
    pub fn construct<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let entity = commands.spawn(self).insert(ButtonBundle {
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::End,
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        }).id();
        commands.entity(entity)
    }
}
#[derive(Component, Default)]
pub struct UiWindowBarButton;

#[derive(Debug, Component)]
pub struct UiButton {
    pub mouse_over: bool,
    pub pressed: bool,
    pub ui_button_action: UiButtonAction,
}
impl Default for UiButton {
    fn default() -> Self {
        Self { mouse_over: false, pressed: false, ui_button_action: UiButtonAction::default() }
    }
}

impl UiButton {
    pub fn construct<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let entity = commands.spawn(self).insert(ButtonBundle {
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 1.0)),
            style: Style {
                width: Val::Px(17.5),
                height: Val::Px(17.5),
                ..default()
            },
            ..default()
        }).id();
        commands.entity(entity)
    }
    pub fn new_close_window() -> Self {
        Self {
            ui_button_action: UiButtonAction::CloseWindow,
            ..default()
        }
    }
}
#[derive(Debug, Default, PartialEq)]
pub enum UiButtonAction {
    #[default]
    None,
    CloseWindow,
}
impl UiButtonAction {
    pub fn is_close_window(&self) -> bool {
        self == &Self::CloseWindow
    }
}

pub struct UiWindowSystemsPlugin;
impl Plugin for UiWindowSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                window_sorting,
                window_movement,

                close_window, // needs to be last
            ).chain())
        ;
    }
}
fn setup(
    mut commands: Commands,
) {
    commands.spawn(UiWindowParent).insert(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    });
}
fn close_window(
    query_button: Query<(&UiButton, &Interaction, &Parent), Changed<Interaction>>,
    query_parent: Query<&Parent, (Without<UiWindow>, Without<UiButton>, With<Node>)>,
    query_parent_window: Query<Entity, With<UiWindow>>,
    mut commands: Commands,
) {
    for (_, _, button_parent) in query_button.iter().filter(|(button, interaction, _)| interaction == &&Interaction::Pressed && button.ui_button_action.is_close_window()) {
        let mut option_parent = Some(button_parent);
        while let Some(parent) = option_parent {
            if let Ok(parent_window) = query_parent_window.get(parent.get()) {
                commands.entity(parent_window).despawn_recursive();
                break
            } else {
                option_parent = query_parent.get(parent.get()).ok();
            }
        }
    }
}
fn window_sorting(
    query_ui_window: Query<(Entity, &Node, &Parent, &AbsoluteInteraction), With<UiWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let mut priority = 0;
        let mut option_entity: Option<Entity> = None;
        let mut option_parent: Option<&Parent> = None;
        for (entity, node, parent, _) in query_ui_window.iter().filter(|(_, _, _, absolute_interaction)| absolute_interaction.hovered()) {
            if node.stack_index() > priority || priority == 0 {
                priority = node.stack_index();
                option_entity = Some(entity);
                option_parent = Some(parent);
            }
        }
        if let (Some(entity), Some(parent)) = (option_entity, option_parent) {
            commands.entity(entity).remove_parent();
            commands.entity(parent.get()).add_child(entity);
        }
    }
}
fn window_movement(
    query_ui_window_bar: Query<(&Interaction, &Parent), (With<UiWindowBar>, Without<UiWindow>)>,
    query_parent: Query<Option<&Parent>, (With<Node>, Without<UiWindow>)>,
    mut query_ui_window: Query<&mut Style, (With<Node>, Without<UiWindowBar>)>,
    mouse_keys: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    if mouse_keys.pressed(MouseButton::Left) {
        for (_, bar_parent) in query_ui_window_bar.iter().filter(|(interaction, _)| interaction == &&Interaction::Pressed) {
            let mut option_parent = Some(bar_parent);
            while let Some(parent) = option_parent {
                if let Some(mut style) = query_ui_window.get_mut(parent.get()).ok() {
                    if let (Val::Px(top), Val::Px(left), Some(cursor_velocity)) = (style.top, style.left, cursor.cursor_velocity()) {
                        style.top = Val::Px(top + cursor_velocity.y);
                        style.left = Val::Px(left + cursor_velocity.x);
                    }
                    break
                } else if let Some(new_parent) = query_parent.get(parent.get()).ok() {
                    option_parent = new_parent;
                } else {
                    break
                }
            }
        }
    }
}