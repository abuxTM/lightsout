use bevy::prelude::*;

use crate::modules::game::{hud::components::{LevelText, XPSlider}, player::components::{Player, PlayerLevel}};

pub fn spawn_xp_slider(mut commands: Commands) {
    let entity = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(40.),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
    )).id();

    let handle = commands.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb_u8(7, 102, 120)),
                ..default()
            },
            XPSlider
    )).id();

    commands.spawn((
            TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(10.),
                ..default()
            },
            text: Text::from_section(
                "LEVEL",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            z_index: ZIndex::Global(1),
            ..default()
        },
        LevelText
    ));

    commands.entity(entity).push_children(&[handle]);
}

pub fn update_xp_slider(
    mut level_query: Query<&mut PlayerLevel, With<Player>>,
    mut slider_query: Query<&mut Style, With<XPSlider>>,
    mut level_text_query: Query<&mut Text, With<LevelText>>
) {
    if let Ok(mut level) = level_query.get_single_mut() {
        if let Ok(mut slider) = slider_query.get_single_mut() {
            if let Ok(mut level_text) = level_text_query.get_single_mut() {
                let xp_percentage = (level.xp as f32 / level.xp_to_level_up as f32) * 100.0;
                slider.width =  Val::Percent(xp_percentage);

                level_text.sections[0].value = format!("Level: {}", level.level);
                level.check_level_up();
            }
        }
    }
}
