use bevy::prelude::*;
use rand::Rng;

use super::spawn::spawn_enemy;

pub fn enemy_spawner(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..120 {
        spawn_enemy(
            &mut commands,
            Transform::from_xyz(
                rng.gen_range(-650.0..650.0),
                rng.gen_range(-650.0..650.0),
                0.,
            ),
        );
    }
}
