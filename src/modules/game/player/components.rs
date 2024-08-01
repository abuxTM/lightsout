use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct PlayerLevel {
    pub level: i32,
    pub xp_to_level_up: i32,
    pub xp: i32
}

impl PlayerLevel {
    pub fn add_level(&mut self, value: i32) {
        self.level += value;
    }

    pub fn set_level(&mut self, value: i32) {
        self.level = value;
    }
    
    pub fn add_xp(&mut self, value: i32) {
        self.xp += value;
        self.check_level_up();
    }

    pub fn set_xp(&mut self, value: i32) {
        self.xp = value;
        self.check_level_up();
    }

    pub fn check_level_up(&mut self) {
        while self.xp >= self.xp_to_level_up {
            self.xp -= self.xp_to_level_up;
            self.level += 1;
            self.update_xp_to_level_up();
        }
    }

    fn update_xp_to_level_up(&mut self) {
        self.xp_to_level_up = (self.level * 100) as i32;
    }
}
