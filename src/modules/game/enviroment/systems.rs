use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Ccd, RigidBody},
    geometry::Collider,
};

pub fn spawn_tils(mut commands: Commands) {
    commands
        .spawn(SpriteBundle::default())
        .with_children(|parent| {
            for x in 0..10 {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(16., 16.)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32 * 16., -64., -1.),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(8., 8.),
                    Ccd::enabled(),
                ));
            }
        });
}
