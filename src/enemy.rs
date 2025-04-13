use std::time::Duration;

use bevy::{math::vec3, prelude::*, time::common_conditions::on_timer};
use rand::{rng, Rng};

use crate::{
    animations::AnimationTimer, player::Player, state::GameState, GlobalTextureAtlas, ENEMY_SPAWN_AT_ONCE, ENEMY_SPAWN_INTERVAL, ENEMY_SPEED, MAX_NUM_ENEMIES, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_enemy)
                .run_if(in_state(GameState::InGame))
                .run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
        )
        .add_systems(Update, approach_player.run_if(in_state(GameState::InGame)));
    }
}

fn approach_player(
    mut enemy_query : Query<&mut Transform,With<Enemy>>,
    player_query: Query<&Transform, (With<Player>,Without<Enemy>)>
){
    if player_query.is_empty() || enemy_query.is_empty(){
        return;
    }
    let player_pos = player_query.single().translation;
    for mut enemyposition in enemy_query.iter_mut(){
        let dir = (player_pos - enemyposition.translation).normalize();
        enemyposition.translation += dir * ENEMY_SPEED;
    }
}

fn spawn_enemy(
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(ENEMY_SPAWN_AT_ONCE);
    if num_enemies >= MAX_NUM_ENEMIES {
        return;
    }

    let mut rng = rng();
    for _ in 0..enemy_spawn_count {
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 10,
                },
            ),
            Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Enemy,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}
