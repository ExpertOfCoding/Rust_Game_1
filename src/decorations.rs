use crate::animations::AnimationTimer;
use crate::constants::*;
use crate::gun::*;
use crate::player::*;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::*;
pub struct WorlDecorationsPlugin;

impl Plugin for WorlDecorationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InitGame),
            (spawn_world_decorations, init_world),
        );
    }
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.random_range(-WORLD_WIDTH..=WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..=WORLD_HEIGHT);
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: rng.random_range(20..=21),
                },
            ),
            Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ));
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(BIGNESS_PLAYER)),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        PlayerState::default(),
    ));
    commands.spawn((
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 16,
            },
        ),
        Transform::from_scale(Vec3::splat(BIGNESS_PLAYER)),
        Gun,
        GunTimer(Stopwatch::new()),
    ));
    next_state.set(GameState::InGame);
}
