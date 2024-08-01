use bevy::{math::FloatOrd, prelude::*, reflect::List};
use rand::Rng;

use crate::modules::game::{
    components::{EntityClass, EntityClasses}, enemy::components::Enemy, spell::components::{SpellBook, SpellCaster, SpellCasterCooldown}
};

use super::spawn::*;

pub fn spell_caster(
    mut commands: Commands,
    mut casters: Query<(&Transform, &SpellCaster, &SpellBook, &mut SpellCasterCooldown)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (caster_transform, caster, spellbook, mut cooldown) in casters.iter_mut() {
        cooldown.0.tick(time.delta());

        if cooldown.0.just_finished() {
            if caster.0 {
                let direction = enemies
                    .iter()
                    .min_by_key(|enemy_transform| {
                        FloatOrd(Vec3::distance(
                            enemy_transform.translation(),
                            caster_transform.translation,
                        ))
                    })
                    .map(|closest_enemy| {
                        closest_enemy.translation() - caster_transform.translation
                    })
                    .unwrap_or_else(|| Vec3::new(1.0, 0.0, 0.0));

                let sound_effects = ["fireball.ogg", "fireball_two.ogg"];
                let sound_effect = sound_effects[rng.gen_range(0..sound_effects.len())];
                let adjusted_transform = Transform::from_xyz(caster_transform.translation.x, caster_transform.translation.y, -1.);

                for spell in &spellbook.spells {
                    if spell == "Fireball" {
                        spawn_fireball_barrage(
                            &mut commands,
                            Transform::from_translation(caster_transform.translation),
                            160.,
                            rng.gen_range(5..8),
                            true,
                            direction,
                            assets.load(sound_effect),
                        );
                    }
                    if spell == "Concentration" {
                        spawn_consecration(
                            &mut commands,
                            Transform::from_translation(adjusted_transform.translation),
                            rng.gen_range(5..8),
                            true,
                            assets.load("fireball.ogg")
                        );
                    }
                }
            }
        }
    }
}
