use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Ccd, LockedAxes, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider},
};

use crate::modules::game::{
    components::Health,
    enemy::components::{Enemy, EnemySpeed},
    spell::components::{SpellCaster, SpellCasterCooldown},
};

pub fn spawn_enemy(commands: &mut ChildBuilder, transform: Transform) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(18.)),
                    color: Color::srgb_u8(46, 104, 230),
                    ..default()
                },
                transform,
                ..default()
            },
            Enemy,
            EnemySpeed(40.),
            Health(20),
            SpellCaster(false),
            SpellCasterCooldown(Timer::from_seconds(1.4, TimerMode::Repeating)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Name::new("Enemy"),
        )).with_children(|parent| {
            parent.spawn((
                Collider::cuboid(8., 8.),
                ActiveEvents::COLLISION_EVENTS,
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
            ));
        });
}
