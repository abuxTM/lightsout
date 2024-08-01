use bevy::{audio::PlaybackMode, prelude::*};
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, Sensor},
};

use crate::modules::game::spell::components::*;

pub fn spawn_fireball(
    commands: &mut Commands,
    transform: Transform,
    speed: f32,
    damage: i32,
    piercing: bool,
    direction: Vec3,
    sound_effect: Handle<AudioSource>,
) {
    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(12.)),
                    color: Color::srgb_u8(222, 35, 51),
                    ..default()
                },
                transform,
                ..default()
            },
            Spell,
            SpellSpeed(speed),
            SpellDamage(damage),
            SpellPiercing(piercing),
            SpellDirection(direction),
            Name::new("Fireball"),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(8., 8.),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
    commands.spawn(AudioBundle {
        source: sound_effect,
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            spatial: true,
            ..default()
        },
    });
}
