use crate::constants::*;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::state::GameState;
pub struct PlayerPlugin;

#[derive(Component,Default)]
pub enum PlayerState{
    #[default]
    Idle,
    Moving
}

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform,&mut PlayerState),With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Bevy 0.12+ için değişti
) {
    if player_query.is_empty() {
        return;
    }
    let (mut transform , mut state) = player_query.single_mut();
    if DEBUG.player_position {
        println!("Transform: {:?}", transform);
    }
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    let mut delta = Vec3::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();
    if delta.is_finite() && (w_key || a_key || s_key || d_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        transform.translation.z = 10.0;
        *state = PlayerState::Moving;
    }else{
        *state = PlayerState::Idle;
    }
}
