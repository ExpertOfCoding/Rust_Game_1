use bevy::app::AppExit;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use rust_game_1::camera::FollowCameraPlugin;
use rust_game_1::decorations::WorlDecorationsPlugin;
use rust_game_1::enemy::EnemyPlugin;
use rust_game_1::gun::GunPlugin;
use rust_game_1::animations::AnimationPlugin;
use rust_game_1::player::PlayerPlugin;
use rust_game_1::state::GameState;
use rust_game_1::{constants::*, ResourcesPlugin};
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My Game".to_string(),
                    resizable: true,
                    focused: true,
                    resolution: (WW, WH).into(),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(FollowCameraPlugin)
    .add_plugins(GunPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(ResourcesPlugin)
    .add_plugins(AnimationPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(WorlDecorationsPlugin)
    .add_systems(Update, close_when_requested)
    .init_state::<GameState>();
    app.run();
}

fn close_when_requested(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Bevy 0.12+ için değişti
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
