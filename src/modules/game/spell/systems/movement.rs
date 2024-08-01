use bevy::prelude::*;

use crate::modules::game::{components::LifeTime, spell::components::*};

pub fn spell_movement(
    mut commands: Commands,
    mut spells: Query<(Entity, &mut Transform, &SpellDirection, &SpellSpeed, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut spell_transform, spell_direction, spell_speed, mut lifetime) in spells.iter_mut() {
        lifetime.0.tick(time.delta());

        spell_transform.translation +=
            spell_direction.0.normalize() * spell_speed.0 * time.delta_seconds();

        if lifetime.0.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn despawn_spells(
    mut commands: Commands,
    mut spells: Query<(Entity, &mut LifeTime), With<Spell>>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in spells.iter_mut() {
        lifetime.0.tick(time.delta());

        if lifetime.0.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct FloatingDamage;

pub fn spawn_floating_damage_text(commands: &mut Commands, transform: Transform, damage: i32) {
    let text_transform = transform.translation;
    let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(damage.to_string(), text_style.clone()),
            transform: Transform::from_xyz(text_transform.x, text_transform.y, 2.),
            ..default()
        },
        FloatingDamage,
        LifeTime(Timer::from_seconds(1., TimerMode::Once)),
        Name::new("Floating Damage")
    ));
}

pub fn update_floating_text(
    mut commands: Commands,
    mut text_query: Query<(Entity, &mut Transform, &mut LifeTime), With<FloatingDamage>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut lifetime) in text_query.iter_mut() {
        lifetime.0.tick(time.delta());

        transform.scale.x -= 1. * time.delta_seconds();
        transform.scale.y -= 1. * time.delta_seconds();

        transform.scale.x = transform.scale.x.max(0.0);
        transform.scale.y = transform.scale.y.max(0.0);

        transform.translation.y += 8. * time.delta_seconds();

        if lifetime.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
