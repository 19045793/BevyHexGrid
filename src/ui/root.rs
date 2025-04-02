// ui/root.rs
use bevy::prelude::*;
use crate::components::*;
use crate::resources::SelectionState;

pub fn setup_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            UiRoot,
            UiBlocking,
        ))
        .with_children(|parent| {
            spawn_player_info_panel(parent, &asset_server);
            spawn_unit_info_panel(parent, &asset_server);
        });
}

fn spawn_player_info_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        },
        PlayerInfoPanel,
        UiBlocking,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Player 1",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn spawn_unit_info_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        },
        UnitInfoPanel,
        UiBlocking,
    ))
    .with_children(|parent| {
        // Unit information text
        parent.spawn((
            TextBundle::from_section(
                "No unit selected",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            UnitInfoText,
        ));

        // Details button
        spawn_details_button(parent);
    });
}

fn spawn_details_button(parent: &mut ChildBuilder) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
            ..default()
        },
        DetailedMenuButton,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "More Details",
            TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

pub fn update_unit_info_system(
    selection_state: Res<SelectionState>,
    unit_query: Query<(&Unit, &UnitType)>,
    mut unit_info_query: Query<&mut Text, With<UnitInfoText>>,
    mut unit_info_panel_query: Query<&mut Style, With<UnitInfoPanel>>,
) {
    let mut unit_info_style = unit_info_panel_query.single_mut();
    
    if let Some(selected_entity) = selection_state.selected_entity {
        if let Ok((unit, unit_type)) = unit_query.get(selected_entity) {
            unit_info_style.display = Display::Flex;
            
            if let Ok(mut text) = unit_info_query.get_single_mut() {
                text.sections[0].value = format!(
                    "Unit: {}\nHealth: {:.1}\nAttack: {:.1}\nMovement Range: {}",
                    unit_type.name(),
                    unit.health,
                    unit.attack_damage,
                    unit.movement_range
                );
            }
        }
    } else {
        unit_info_style.display = Display::None;
        if let Ok(mut text) = unit_info_query.get_single_mut() {
            text.sections[0].value = "No unit selected".to_string();
        }
    }
}