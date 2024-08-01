use bevy::{math::FloatOrd, prelude::*};
use rand::Rng;

use crate::modules::game::{
    enemy::components::Enemy,
    spell::components::{SpellCaster, SpellCasterCooldown},
};

use super::spawn::spawn_fireball;

pub fn spell_caster(
    mut commands: Commands,
    mut casters: Query<(&Transform, &SpellCaster, &mut SpellCasterCooldown)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (caster_transform, caster, mut cooldown) in casters.iter_mut() {
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
                    });

                let sound_effects = ["fireball.ogg", "fireball_two.ogg"];
                let sound_effect = sound_effects[rng.gen_range(0..sound_effects.len())];

                spawn_fireball(
                    &mut commands,
                    Transform::from_translation(caster_transform.translation),
                    100.,
                    rng.gen_range(4..6),
                    true,
                    direction.unwrap_or(Vec3::new(1., 0., 0.)),
                    assets.load(sound_effect),
                );
            }
        }
    }
}
