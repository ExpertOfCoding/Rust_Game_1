use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::input::keyboard::KeyCode;
use bevy::math::vec3;
use bevy::prelude::*; // AppExit event'i için gerekli
// ComputedVisibility is already included in bevy::prelude

// DEBUG
struct Debug {
    cursor_position: bool,
    player_position: bool,
}

const DEBUG: Debug = Debug {
    cursor_position: true,
    player_position: false,
};

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

// Sprite sheet
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SHEET_WIDTH: u32 = 4;
const SPRITE_SHEET_HEIGHT: u32 = 4;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const BIGNESS_PLAYER : f32 = 3.0;
// PLAYER
const PLAYER_SPEED: f32 = 3.0;

// Colors
fn bg_color() -> Color {
    Color::srgb_u8(28, 28, 28)
}

// Game States
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    InitGame,
    InGame,
}

#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
struct CursorPosition(Option<Vec2>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

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
    .init_state::<GameState>()
    .insert_resource(ClearColor(bg_color()))
    .insert_resource(CursorPosition(None))
    .insert_resource(GlobalSpriteSheetHandle(None))
    .insert_resource(GlobalTextureAtlasHandle(None))
    .add_systems(OnEnter(GameState::Loading), load_assets)
    .add_systems(OnEnter(GameState::InitGame), (setup, init_world))
    .add_systems(
        Update,
        (
            update_gun_transform,
            handle_player_input,
            update_cursor_position,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .add_systems(Update, close_when_requested);
    app.run();
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Check if either query is empty and handle it properly
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_position.0 = None;
        return; // Early return to avoid the code below
    }

    // Now it's safe to use single() since we've confirmed the queries aren't empty
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    
    cursor_position.0 = window
        .cursor_position()
        .and_then(|cursor| {
            match camera.viewport_to_world(camera_transform, cursor) {
                Ok(ray) => Some(ray.origin.truncate()),
                Err(_) => None,
            }
        })
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handler: ResMut<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handler.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH, TILE_HEIGHT),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));
    next_state.set(GameState::InitGame);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handler: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            image_handler.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(BIGNESS_PLAYER)),
        Player,
    ));
    commands.spawn((
        Sprite::from_atlas_image(
            image_handler.0.clone().unwrap(),
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 1,
            },
        ),
        Transform::from_scale(Vec3::splat(BIGNESS_PLAYER)),
        Gun,
    ));
    next_state.set(GameState::InGame);
}

fn handle_player_input(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Bevy 0.12+ için değişti
) {
    if player_query.is_empty() {
        return;
    }
    let mut transform = player_query.single_mut().1;
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
    }
}

fn update_gun_transform(
    cursor_position: Res<CursorPosition>,
    mut param_set: ParamSet<(Query<(&Player, &Transform)>, Query<(&Gun, &mut Transform)>)>,
) {
    // Get player position first
    let player_position = if let Ok((_, player_transform)) = param_set.p0().get_single() {
        player_transform.translation
    } else {
        return; // Early return if no player found
    };

    // Check if cursor position exists
    let cursor_pos = match cursor_position.0 {
        Some(pos) => pos,
        None => return, // Early return if no cursor position
    };

    // Then update gun position
    if let Ok((_, mut gun_transform)) = param_set.p1().get_single_mut() {
        // Calculate the angle between the player and the cursor
        let angle = (player_position.y - cursor_pos.y )
            .atan2(player_position.x-cursor_pos.x ) + PI;
        // Calculate the new gun position with an offset
        let offset = 20.0;
        let new_gun_pos = Vec2::new(
            player_position.x + offset * angle.cos() -5.0,
            player_position.y + offset * angle.sin() -10.0,
        );

        // Update the gun's translation
        gun_transform.translation = Vec3::new(
            new_gun_pos.x,
            new_gun_pos.y,
            gun_transform.translation.z,
        );
        gun_transform.rotation = Quat::from_rotation_z(angle);


        if DEBUG.cursor_position {  // Fixed field name
            println!("Cursor Position: {:?}", cursor_position.0);
        }
    }
}

fn close_when_requested(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Bevy 0.12+ için değişti
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
