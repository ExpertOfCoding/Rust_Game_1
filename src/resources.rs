use crate::constants::*;
use crate::state::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(bg_color()))
            .insert_resource(CursorPosition(None))
            .insert_resource(GlobalTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut handle: ResMut<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH, TILE_HEIGHT),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));
    next_state.set(GameState::InitGame);
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

    cursor_position.0 = window.cursor_position().and_then(|cursor| {
        match camera.viewport_to_world(camera_transform, cursor) {
            Ok(ray) => Some(ray.origin.truncate()),
            Err(_) => None,
        }
    })
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}
