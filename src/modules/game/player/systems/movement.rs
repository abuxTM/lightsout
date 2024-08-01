use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::modules::game::player::components::*;

pub fn movement(
    mut player_query: Query<(&mut Sprite, &PlayerSpeed, &mut Velocity), With<Player>>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut sprite, speed, mut velocity)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if inputs.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if inputs.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if inputs.pressed(KeyCode::KeyA) {
            sprite.flip_x = true;
            direction.x -= 1.;
        }
        if inputs.pressed(KeyCode::KeyD) {
            sprite.flip_x = false;
            direction.x += 1.;
        }

        let move_delta = direction.normalize_or_zero() * speed.value;
        velocity.linvel = move_delta;
    }
}
