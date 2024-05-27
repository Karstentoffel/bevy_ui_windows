use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::ui::FocusPolicy;
use crate::ui_window::*;
use bevy_cursor_tools::*;

#[derive(Debug, Component)]
pub struct ContextMenu(pub ContextMenuOptions, pub Option<Entity>);
impl Default for ContextMenu {
    fn default() -> Self {
        Self(ContextMenuOptions::EMPTY, None)
    }
}
impl ContextMenu {
    pub fn bundle(self) -> ContextMenuBundle {
        ContextMenuBundle {
            context_menu: self,
            ..default()
        }
    }
    pub fn construct<'a>(self, commands: &'a mut Commands, translation: Vec2) -> EntityCommands<'a> {
        let mut children: Vec<Entity> = Vec::new();
        for option in &self.0.0 {
            let option_button = commands.spawn(ContextMenuEventButton(*option)).insert(ButtonBundle{ background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)), ..ButtonBundle::default() }).id();
            let option_text = commands.spawn(TextBundle { focus_policy: FocusPolicy::Pass, ..TextBundle::from(option.0) }).id();

            commands.entity(option_button).push_children(&[option_text]);

            children.push(option_button);
        }
        let entity = UiWindowBundle::default().with_style(Style { top: Val::Px(translation.y), left: Val::Px(translation.x), ..ContextMenuBundle::default_style() }).construct_borderless(commands).insert(self).insert(Interaction::None).insert(Button::default()).id();
        commands.entity(entity).push_children(&children);
        commands.entity(entity)
    }
}

#[derive(Debug, Bundle)]
pub struct ContextMenuBundle {
    pub context_menu: ContextMenu,
    pub node_bundle: NodeBundle,
}
impl Default for ContextMenuBundle {
    fn default() -> Self {
        Self { context_menu: ContextMenu::default(), node_bundle: ContextMenuBundle::default_node_bundle() }
    }
}
impl ContextMenuBundle {
    fn default_style() -> Style {
        Style { position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..default()
        }
    }
    fn default_node_bundle() -> NodeBundle {
        NodeBundle {
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.2)),
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    }
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.node_bundle.style.top = Val::Px(position.y);
        self.node_bundle.style.left = Val::Px(position.x);
        self
    }
}

#[derive(Debug, Component)]
pub struct ContextMenuEventButton(pub ContextMenuOption);

#[derive(Debug, Component, Clone)]
pub struct ContextMenuOptions (pub Vec<ContextMenuOption>);
impl ContextMenuOptions {
    pub const EMPTY: Self = Self(Vec::new());
    pub fn canvas() -> Self {
        Self(vec![ContextMenuOption("Spawn")])
    }
    pub fn with(mut self, options: Vec<ContextMenuOption>) -> Self {
        self.0.extend(options);
        self
    }
    pub fn from_options(options: Vec<ContextMenuOption>) -> Self {
        Self(options)
    }
    pub fn bundle(self) -> ContextMenuOptionsBundle {
        ContextMenuOptionsBundle {
            context_menu_options: self,
            ..default()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ContextMenuOption(pub &'static str);

#[derive(Debug, Bundle)]
pub struct ContextMenuOptionsBundle {
    pub context_menu_options: ContextMenuOptions,
    /// Marker component that signals this node is a button
    pub button: Button,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
}
impl Default for ContextMenuOptionsBundle {
    fn default() -> Self {
        Self { context_menu_options: ContextMenuOptions::EMPTY, button: Button::default(), interaction: Interaction::None }
    }
}

pub struct ContextMenuSystemsPlugin;
impl Plugin for ContextMenuSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                context_menu,
                context_menu_cleanup,
                context_menu_button,
            ))
        ;
    }
}
fn context_menu(
    query_node_context_menu_options: Query<(Entity, &Interaction), (With<ContextMenuOptions>, With<Node>)>,
    query_object_context_menu_options: Query<(Entity, &Transform), (Without<Node>, With<ContextMenuOptions>)>,
    query_context_menu_options: Query<&ContextMenuOptions>,
    query_context_menu: Query<&ContextMenu>,
    keys: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut commands: Commands,
) {
    if keys.just_pressed(MouseButton::Right) {
        let mut clicked_entity: Option<Entity> = None;
        for (entity, _) in query_node_context_menu_options.iter().filter(|(_, interaction)| interaction == && Interaction::Hovered) {
            clicked_entity = Some(entity)
        }
        if clicked_entity.is_none() {
            for (entity, transform) in query_object_context_menu_options.iter() {
                // The LevelObject Interaction
            }
        }
        if let Some(context_menu_options) = clicked_entity.and_then(|entity| query_context_menu_options.get(entity).ok()) {
            // Here the clicked_entity is always Some.
            ContextMenu(context_menu_options.clone(), clicked_entity).construct(&mut commands, cursor.valid_position);
        } else if query_context_menu.is_empty() {
            ContextMenu(ContextMenuOptions::canvas(), None).construct(&mut commands, cursor.valid_position);
        }
    }
}

fn context_menu_cleanup(
    query_context_menu: Query<(Entity, &Interaction), With<ContextMenu>>,
    keys: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if keys.get_just_pressed().len() > 0 {
        for (entity, _) in query_context_menu.iter().filter(|(_, interaction)| interaction == && Interaction::None) {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn context_menu_button(
    mut query: Query<(&mut BackgroundColor, &Interaction, &ContextMenuEventButton), Changed<Interaction>>,
) {
    for (mut color, interaction, context_button) in query.iter_mut() {
        color.0 = match interaction {
            Interaction::None => Color::rgba(0.0, 0.0, 0.0, 0.0),
            _ => Color::rgba(0.0, 0.0, 0.0, 0.2),
        };
    }
}