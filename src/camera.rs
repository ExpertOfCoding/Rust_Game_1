use crate::player::Player;
use crate::state::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::InitGame), setup)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2d::default())
        .insert(PanCam::default());
}

fn camera_follow_player(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().1.translation;
    let (x, y) = (player_transform.x, player_transform.y);
    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}
