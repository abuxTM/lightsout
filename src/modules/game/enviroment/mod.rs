use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct EnviromentPlugin;

impl Plugin for EnviromentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tils);
    }
}
