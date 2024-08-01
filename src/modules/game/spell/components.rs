use bevy::prelude::*;

#[derive(Component)]
pub struct SpellCaster(pub bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpellCasterCooldown(pub Timer);

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct Spell;

#[derive(Component)]
pub struct SpellEnabled(pub bool);

#[derive(Component)]
pub struct SpellPiercing(pub bool);

#[derive(Component)]
pub struct SpellDamage(pub i32);

#[derive(Component)]
pub struct SpellSpeed(pub f32);

#[derive(Component)]
pub struct SpellDirection(pub Vec3);