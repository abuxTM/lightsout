use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::modules::game::{
    player::components::*,
    spell::components::{SpellCaster, SpellCasterCooldown},
};

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(15., 24.)),
                    ..default()
                },
                texture: asset_server.load("player.png"),
                ..default()
            },
            Player,
            PlayerSpeed { value: 80. },
            SpellCaster(true),
            SpellCasterCooldown(Timer::from_seconds(1., TimerMode::Repeating)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Ccd::enabled(),
            Velocity::zero(),
            Collider::capsule_y(15. / 3., 24. / 3.),
        ))
        .with_children(|parent| {
            parent.spawn(Camera2dBundle::default());
        });
}
