use bevy::prelude::*;
use components::SpellCasterCooldown;
use systems::{
    movement::{handle_collisions, spell_movement, update_floating_text},
    spell_caster::spell_caster,
};

pub mod components;
pub mod systems;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spell_caster,
                spell_movement,
                handle_collisions,
                update_floating_text,
            ),
        )
        .register_type::<SpellCasterCooldown>();
    }
}
