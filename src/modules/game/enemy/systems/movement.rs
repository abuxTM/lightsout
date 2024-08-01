use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::modules::game::{
    components::Health,
    enemy::components::{Enemy, EnemySpeed},
    player::components::{Player, PlayerLevel},
};

pub fn enemy_movement(
    mut enemies: Query<(&Transform, &mut Velocity, &EnemySpeed), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    for (enemy_transform, mut enemy_velocity, enemy_speed) in enemies.iter_mut() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        let dir = (player_transform.translation.truncate()
            - enemy_transform.translation.truncate())
        .normalize_or_zero();

        enemy_velocity.linvel = dir * enemy_speed.0;
    }
}

pub fn kill_enemy(
    mut commands: Commands,
    enemies: Query<(Entity, &Health), With<Enemy>>,
    mut player_query: Query<&mut PlayerLevel, With<Player>>
) {
    for (entity, health) in enemies.iter() {
        if let Ok(mut player_level) = player_query.get_single_mut() {
            if health.0 <= 0 {
                commands.entity(entity).despawn_recursive();
                player_level.xp += 10;
            }
        }
    }
}
