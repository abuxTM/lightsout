use bevy::prelude::*;

#[derive(Component)]
pub struct EntityClass {
    pub class: EntityClasses
}

#[derive(PartialEq)]
pub enum EntityClasses {
    Warrior,
    Paladin,
    Mage,
}

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct LifeTime(pub Timer);
