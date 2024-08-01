use bevy::prelude::*;
use enemy::EnemyPlugin;
use enviroment::EnviromentPlugin;
use player::PlayerPlugin;
use spell::SpellPlugin;

mod components;
mod enemy;
mod enviroment;
mod player;
mod spell;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin, SpellPlugin, EnviromentPlugin));
    }
}
