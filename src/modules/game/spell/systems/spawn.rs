use std::f32::consts::PI;

use bevy::{audio::PlaybackMode, prelude::*};
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, Sensor},
};

use crate::modules::game::{components::LifeTime, spell::components::*};

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
            LifeTime(Timer::from_seconds(1.5, TimerMode::Once)),
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
    /*
    commands.spawn(AudioBundle {
        source: sound_effect,
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            //spatial: true,
            ..default()
        },
    });
     */
}

pub fn spawn_fireball_barrage(
    commands: &mut Commands,
    transform: Transform,
    speed: f32,
    damage: i32,
    piercing: bool,
    direction: Vec3,
    sound_effect: Handle<AudioSource>,
) {
    let angle_offset = PI / 12.0;

    for i in 0..8 {
        let angle = angle_offset * (i as f32 - 1.5);
        let rotation = Quat::from_rotation_z(angle);
        let new_direction = rotation * direction;

        let new_transform = Transform {
            translation: transform.translation,
            rotation,
            ..transform
        };

        spawn_fireball(
            commands,
            new_transform,
            speed,
            damage,
            piercing,
            new_direction,
            sound_effect.clone(),
        );
    }
}

pub fn spawn_consecration(
    commands: &mut Commands,
    transform: Transform,
    damage: i32,
    piercing: bool,
    sound_effect: Handle<AudioSource>,
) {
    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(80.)),
                    color: Color::srgb_u8(242, 212, 41),
                    ..default()
                },
                transform,
                ..default()
            },
            Spell,
            SpellDamage(damage),
            SpellPiercing(piercing),
            LifeTime(Timer::from_seconds(3.5, TimerMode::Once)),
            Name::new("Fireball"),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(40., 40.),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}
