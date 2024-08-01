use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_rapier2d::{dynamics::Velocity, geometry::Collider, na::clamp, pipeline::CollisionEvent};
use rand::Rng;

use crate::modules::game::{components::Health, enemy::components::Enemy, spell::components::*};

use super::movement::spawn_floating_damage_text;

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
