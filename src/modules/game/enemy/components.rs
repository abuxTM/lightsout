use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpeed(pub f32);

#[derive(Component)]
pub struct EnemyState(pub EnemyStates);

#[derive(Default)]
pub enum EnemyStates {
    #[default]
    Idle,
    Walking,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
