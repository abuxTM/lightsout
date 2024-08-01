use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_rapier2d::{dynamics::Velocity, geometry::Collider, na::clamp, pipeline::CollisionEvent};
use rand::Rng;

use crate::modules::game::{
    components::Health,
    enemy::components::Enemy,
    spell::components::{Spell, SpellDamage, SpellDirection, SpellPiercing, SpellSpeed},
};

pub fn spell_movement(
    mut spells: Query<(&mut Transform, &SpellDirection, &SpellSpeed)>,
    time: Res<Time>,
) {
    for (mut spell_transform, spell_direction, spell_speed) in spells.iter_mut() {
        spell_transform.translation +=
            spell_direction.0.normalize() * spell_speed.0 * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct FloatingDamage;

#[derive(Component)]
pub struct LifeTime(pub Timer);

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

pub fn handle_collisions(
    mut commands: Commands,
    mut spells: Query<(Entity, &SpellDamage, &SpellPiercing), With<Spell>>,
    mut enemies: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    colliders: Query<&Parent, (With<Collider>, Without<Enemy>)>,
    mut collision_events: EventReader<CollisionEvent>,
    assets: Res<AssetServer>,
) {
    for event in collision_events.read() {
        let (source, target) = match event {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match colliders.get(*source) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };
        let target_parent = match colliders.get(*target) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };

        let (enemy_entity, mut health, enemy_transform) =
            if let Ok(h) = enemies.get_mut(source_parent) {
                h
            } else if let Ok(h) = enemies.get_mut(target_parent) {
                h
            } else {
                continue;
            };

        let (spell_entity, spell_damage, spell_piercing) =
            if let Ok(f) = spells.get_mut(source_parent) {
                f
            } else if let Ok(f) = spells.get_mut(target_parent) {
                f
            } else {
                continue;
            };

        let mut rng = rand::thread_rng();
        let sound_effects = ["hit.ogg", "hit.ogg"];
        let sound_effect = sound_effects[rng.gen_range(0..sound_effects.len())];

        commands.spawn(AudioBundle {
            source: assets.load(sound_effect),
            settings: PlaybackSettings {
                volume: Volume::new(rng.gen_range(0.5..1.0)),
                mode: PlaybackMode::Despawn,
                spatial: true,
                ..default()
            },
        });
        spawn_floating_damage_text(&mut commands, *enemy_transform, spell_damage.0);
        health.0 -= spell_damage.0;
        if !spell_piercing.0 {
            commands.entity(spell_entity).despawn_recursive();
        }
    }
}
