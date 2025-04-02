// ui/menu.rs (new file for menu-related systems)

use bevy::prelude::*;
use crate::resources::{ContextMenuState, DetailedMenuState, SelectionState};
use crate::components::*;

#[derive(Component)]
pub struct ContextMenu;

#[derive(Component)]
pub struct MenuItem {
    pub action: MenuAction,
}

pub enum MenuAction {
    Move,
    Attack,
    CreateUnit,
    Close,
    ShowDetails,
    ViewDetails,  // Added this variant
}

pub fn handle_context_menu(
    mut commands: Commands,
    mut context_menu_state: ResMut<ContextMenuState>,
    selection_state: Res<SelectionState>,
    mouse_buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    unit_query: Query<&Unit>,
    tile_query: Query<&Tile>,
    existing_menu: Query<Entity, With<ContextMenu>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        // First, remove any existing context menu
        for entity in existing_menu.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if let Some(cursor_pos) = windows.single().cursor_position() {
            context_menu_state.position = cursor_pos;
            spawn_context_menu(
                &mut commands,
                &context_menu_state,
                &selection_state,
                &unit_query,
                &tile_query,
            );
        }
    }
}

pub fn spawn_context_menu(
    commands: &mut Commands,
    context_menu_state: &ContextMenuState,
    selection_state: &SelectionState,
    unit_query: &Query<&Unit>,
    tile_query: &Query<&Tile>,
) {
    println!("Spawning context menu at: {:?}", context_menu_state.position);

    let menu_items = if let Some(entity) = context_menu_state.target_entity {
        // Determine menu items based on entity type
        if unit_query.contains(entity) {
            vec![
                (MenuAction::Move, "Move", Color::WHITE),
                (MenuAction::Attack, "Attack", Color::RED),
                (MenuAction::ShowDetails, "Details", Color::CYAN),  // Changed to ShowDetails
            ]
        } else if tile_query.contains(entity) {
            vec![
                (MenuAction::CreateUnit, "Create Unit", Color::GREEN),
                (MenuAction::ViewDetails, "View Tile", Color::CYAN),
            ]
        } else {
            vec![(MenuAction::CreateUnit, "Create Unit", Color::GREEN)]
        }
    } else {
        vec![(MenuAction::CreateUnit, "Create Unit", Color::GREEN)]
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(context_menu_state.position.x),
                top: Val::Px(context_menu_state.position.y),
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(120.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
            border_color: BorderColor(Color::WHITE),
            z_index: ZIndex::Global(100),
            ..default()
        },
        ContextMenu,
        UiBlocking,
    ))
    .with_children(|parent| {
        for (action, text, color) in menu_items {
            spawn_menu_item(parent, action, text, color);
        }
    });
}

// Detailed menu systems
pub fn handle_detailed_menu(
    mut commands: Commands,
    mut detailed_menu_state: ResMut<DetailedMenuState>,
    selection_state: Res<SelectionState>,
    unit_query: Query<(&Unit, &UnitType)>,
    button_interaction: Query<&Interaction, (Changed<Interaction>, With<DetailedMenuButton>)>,
    windows: Query<&Window>,
) {
    for interaction in button_interaction.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(selected_entity) = selection_state.selected_entity {
                if let Ok((unit, unit_type)) = unit_query.get(selected_entity) {
                    detailed_menu_state.is_open = true;
                    detailed_menu_state.unit_entity = Some(selected_entity);
                    
                    // Position menu near the cursor or center of screen
                    let position = windows.single().cursor_position()
                        .unwrap_or(Vec2::new(400.0, 300.0));
                    
                    spawn_detailed_menu(
                        &mut commands,
                        position,
                        selected_entity,
                        unit,
                        unit_type
                    );
                }
            }
        }
    }
}

fn spawn_detailed_menu(
    commands: &mut Commands,
    position: Vec2,
    entity: Entity,
    unit: &Unit,
    unit_type: &UnitType,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(300.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
            border_color: BorderColor(Color::WHITE),
            z_index: ZIndex::Global(101),
            ..default()
        },
        DetailedMenu,
        UiBlocking,
    ))
    .with_children(|parent| {
        // Header
        parent.spawn(TextBundle::from_section(
            format!("{} Details", unit_type.name()),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Stats
        spawn_detailed_stats(parent, unit);

        // Close button
        spawn_close_button(parent);
    });
}

fn spawn_detailed_stats(parent: &mut ChildBuilder, unit: &Unit) {
    parent.spawn((
        TextBundle::from_sections([
            TextSection::new(
                format!(
                    "\nHealth: {:.1}/{:.1}\nAttack: {:.1}\nMovement: {}\n",
                    unit.health,
                    100.0, // Max health
                    unit.attack_damage,
                    unit.movement_range
                ),
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ]),
        DetailedMenuText,
    ));
}

fn spawn_close_button(parent: &mut ChildBuilder) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(30.0),
                margin: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
            ..default()
        },
        DetailedMenuButton,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Close",
            TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

// Menu interaction system
pub fn handle_menu_interaction(
    mut commands: Commands,
    mut context_menu_state: ResMut<ContextMenuState>,
    mut detailed_menu_state: ResMut<DetailedMenuState>,
    menu_interaction: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    menu_query: Query<Entity, Or<(With<ContextMenu>, With<DetailedMenu>)>>,
) {
    for (interaction, menu_item) in menu_interaction.iter() {
        if *interaction == Interaction::Pressed {
            match menu_item.action {
                MenuAction::Close => {
                    // Close any open menu
                    for entity in menu_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    context_menu_state.is_open = false;
                    detailed_menu_state.is_open = false;
                },
                // Handle other menu actions...
                _ => {}
            }
        }
    }
}

pub fn handle_menu_actions(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    mut context_menu_state: ResMut<ContextMenuState>,
    menu_query: Query<Entity, With<ContextMenu>>,
) {
    for (interaction, menu_item) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match menu_item.action {
                MenuAction::Move => {
                    println!("Move action selected");
                    // Implement move logic
                }
                MenuAction::Attack => {
                    println!("Attack action selected");
                    // Implement attack logic
                }
                MenuAction::CreateUnit => {
                    println!("Create unit action selected");
                    // Implement unit creation logic
                }
                MenuAction::ViewDetails => {
                    println!("View details selected");
                    // Implement details view logic
                }
                MenuAction::ShowDetails => {
                    println!("Show details selected");
                    // Implement show details logic
                }
                MenuAction::Close => {
                    // Close the menu
                    for entity in menu_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    context_menu_state.is_open = false;
                }
            }
            
            // Close menu after any action
            for entity in menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            context_menu_state.is_open = false;
        }
    }
}

fn spawn_menu_item(
    parent: &mut ChildBuilder,
    action: MenuAction,
    text: &str,
    color: Color,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                padding: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        },
        MenuItem { action },
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 16.0,
                color,
                ..default()
            },
        ));
    });
}