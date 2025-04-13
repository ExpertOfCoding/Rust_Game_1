use bevy::color::Color;

// DEFINITIONS
pub const NUM_WORLD_DECORATIONS: usize = 1000;
pub const WORLD_HEIGHT: f32 = 3000.0;
pub const WORLD_WIDTH: f32 = 3000.0;

// DEBUG
pub struct Debug {
    pub cursor_position: bool,
    pub player_position: bool,
}

pub const DEBUG: Debug = Debug {
    cursor_position: true,
    player_position: false,
};

// Window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 700.0;
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;

// GUn TimeOut
pub const GUN_TIMEOUT: f32 = 1.0;

// Bullet Speed
pub const BULLET_SPEED: f32 = 6.219120383494598549854985498203;

// Sprite sheet
pub const SPRITE_SHEET_PATH: &str = "assets2.png";
pub const SPRITE_SHEET_WIDTH: u32 = 5;
pub const SPRITE_SHEET_HEIGHT: u32 = 5;
pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;
pub const BIGNESS_PLAYER: f32 = 3.0;
// PLAYER
pub const PLAYER_SPEED: f32 = 3.0;

pub fn bg_color() -> Color {
    Color::srgb_u8(200, 204, 180)
}

// ENEMY

pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const MAX_NUM_ENEMIES: usize = 800;
pub const ENEMY_SPAWN_AT_ONCE: usize = 20;
pub const ENEMY_SPEED : f32 = 1.1;
