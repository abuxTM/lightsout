use bevy::prelude::*;
use systems::{
    enemy_spawner::enemy_spawner,
    movement::{enemy_movement, kill_enemy},
};

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_spawner)
            .add_systems(Update, (enemy_movement, kill_enemy));
    }
}
