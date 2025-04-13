use bevy::prelude::*;

use crate::{enemy::Enemy, gun::Gun, player::{Player, PlayerState}, state::GameState, CursorPosition, SPRITE_SHEET_WIDTH};


#[derive(Component,Deref,DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_player,tick_animator,animate_enemy,flip_gun_y).run_if(in_state(GameState::InGame)));
    }
}

fn tick_animator(
    time: Res<Time>,
    mut animation_query : Query<&mut AnimationTimer,With<AnimationTimer>>
){
    for mut timer in animation_query.iter_mut(){
        timer.tick(time.delta());
    }
}

fn animate_player(
    cursorpos : Res<CursorPosition>,
    mut player_query : Query<(&mut Sprite,&mut AnimationTimer,&PlayerState, &mut Transform), With<Player>>
){
    if player_query.is_empty(){
        return;
    }
    let (mut texture, timer,state,transform) = player_query.single_mut();
    if timer.finished(){
        let base_sprite_index :usize = match state {
            PlayerState::Idle => 5 as usize,
            PlayerState::Moving => 0 as usize,
        };
        texture.texture_atlas.as_mut().unwrap().index =base_sprite_index + (texture.texture_atlas.as_mut().unwrap().index + 1 ) % SPRITE_SHEET_WIDTH as usize;
    }
    if let Some(cursor_position) = cursorpos.0{
        if cursor_position.x < transform.translation.x{ 
            texture.flip_x = true;
        }else{
            texture.flip_x=false;
        }
    }
}

fn animate_enemy(
 player_query : Query<&Transform,With<Player>>,
 mut enemy_query: Query<(&mut Sprite,&mut AnimationTimer,&mut Transform),(With<Enemy>,Without<Player>)>
){
    if player_query.is_empty() || enemy_query.is_empty(){
        return;
    }

    let player_pos = player_query.single().translation;

    for (mut sprite, timer, transform) in enemy_query.iter_mut(){
        if timer.finished(){
            sprite.texture_atlas.as_mut().unwrap().index = (sprite.texture_atlas.as_mut().unwrap().index + 1 ) % 5 + 10;
        }
        if transform.translation.x < player_pos.x{
            sprite.flip_x = false;
        }else{
            sprite.flip_x = true;
        }
    }
    
}

fn flip_gun_y(
    cursorpos : Res<CursorPosition>,
    mut gun_query : Query<(&mut Sprite, &mut Transform), With<Gun>>
){
    if gun_query.is_empty(){
        return;
    }
    let (mut texture,transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursorpos.0{
        if cursor_position.x < transform.translation.x{
            texture.flip_y = true;
        }else{
            texture.flip_y=false;
        }
    }
}