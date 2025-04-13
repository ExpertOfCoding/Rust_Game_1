use crate::constants::*;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::resources::*;
use crate::state::*;
use bevy::input::mouse;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::f32::consts::PI;
pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletDirection(Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_gun_input, update_gun_transform, update_bullets)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_gun_input(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<mouse::MouseButton>>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
    time: Res<Time>,
) {
    if gun_query.is_empty() {
        return;
    }
    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());
    let bullet_direction = gun_transform.local_x();
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    if gun_timer.0.elapsed_secs() >= GUN_TIMEOUT {
        gun_timer.0.reset();
        commands.spawn((
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 15,
                },
            ),
            Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(*bullet_direction),
        ));
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
        let angle = (player_position.y - cursor_pos.y).atan2(player_position.x - cursor_pos.x) + PI;
        // Calculate the new gun position with an offset
        let offset = 20.0;
        let new_gun_pos = Vec2::new(
            player_position.x + offset * angle.cos() - 5.0,
            player_position.y + offset * angle.sin() - 10.0,
        );

        // Update the gun's translation
        gun_transform.translation =
            Vec3::new(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
        gun_transform.rotation = Quat::from_rotation_z(angle);
        gun_transform.translation.z = 10.0;

        if DEBUG.cursor_position {
            // Fixed field name
            println!("Cursor Position: {:?}", cursor_position.0);
        }
    }
}

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection),With<Bullet>>,
) {
    if bullet_query.is_empty() {
        return;
    }
    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
        t.translation.z = 10.0;

    }
}
