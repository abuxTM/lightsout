use bevy::prelude::*;
use systems::spawn::*;

pub mod components;
pub mod systems;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_xp_slider)
            .add_systems(Update, update_xp_slider);
    }
}
