use bevy::prelude::*;
use physics::*;
use enemies::*;
use gamestate::{GameState};
use bevy::sprite::collide_aabb::collide;

use crate::{Materials, WinSize, TIME_STEP};
use crate::{Animation, LoopAnim};
use crate::{PLAYER_SIZE, ENEMY_SIZE};
use crate::physics;
use crate::enemies;
use crate::gamestate;

// region:    Resources
pub struct PlayerAlive(pub bool);
// endregion:    Resources

// region:    Components
pub struct Player;
struct DeadPlayer;
// endregion:    Components

// region:    Plugin
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(player_spawn.system())
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(reset_player_pos.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_check_collision.system())
                    .with_system(player_dead_movement.system())
                    .with_system(animate_dead_player.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Dead)
                    .with_system(player_spawn.system())
                    .with_system(player_dead_cleanup.system())
            )
            .add_system(player_movement.system())
            .add_system(player_jump.system())
            .insert_resource(PlayerAlive(true));
    }
}
// endregion:    Plugin

//Player Alive

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>
){
    let bottom = - win_size.h/2. + 160.0;
    let left = - win_size.w/2. + 135.0;
    
    commands
        .spawn_bundle(SpriteSheetBundle  {
            texture_atlas: materials.player_a.clone(),
            transform: Transform{
                translation: Vec3::new(left, bottom, 10.),
                scale: Vec3::new(0.30, 0.30, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(AffectedByGravity(false))
        .insert(Timer::from_seconds(0.07, true))
        .insert(Animation{index: 0,size: 16})
        .insert(LoopAnim)
        .insert(Velocity(Vec2::ZERO));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    alive: Res<PlayerAlive>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Animation, 
        &AffectedByGravity, With<Player>)>
){
    if let Ok((mut velocity, mut transform,mut anim, in_air, _)) = query.single_mut() {
        
        if !alive.0 {return;}

        if keyboard_input.pressed(KeyCode::A){
            if transform.scale.x > 0. {
                transform.scale.x = -transform.scale.x;
            }
            velocity.0.x = -300.;
        } else if keyboard_input.pressed(KeyCode::D){
            if transform.scale.x < 0. {
                transform.scale.x = -transform.scale.x;
            }
            velocity.0.x = 300.;
        } else {
            if !in_air.0 {
                anim.index = 0;
                anim.size = 16;
            }
            velocity.0.x = 0.;
        };

        if velocity.0.x != 0.0 && !in_air.0 {
            anim.index = 16;
            anim.size = 20;
        }
        
        transform.translation.y += velocity.0.y * TIME_STEP;
        transform.translation.x += velocity.0.x * TIME_STEP;

        let limit = win_size.w/2.0 - PLAYER_SIZE.0 * transform.scale[0].abs()/2.;
        transform.translation.x = transform.translation.x.min(limit).max(-limit);
    }
}

fn player_jump(
    kb: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    alive: Res<PlayerAlive>,
    mut query: Query<(&mut Transform, &mut Animation, 
        &mut Velocity, &mut AffectedByGravity, With<Player>)>
){
    if let Ok((mut transform, mut anim, mut velocity, mut in_air, _)) = query.single_mut() {
        if !alive.0 {return;}

        if !in_air.0 && (kb.pressed(KeyCode::W)||kb.pressed(KeyCode::Space)) {
            in_air.0 = true;
            anim.index = 36;
            anim.size = 30;

            velocity.0.y = 19.*30.;
        }
        
        if transform.translation.y <  (- win_size.h/2. + 160.0) {
            velocity.0.y = 0.;
            transform.translation.y = - win_size.h/2. + 160.0;
            in_air.0 = false;
        }
    }
}

fn player_check_collision(
    mut commands: Commands,
    materials: Res<Materials>,
    mut alive: ResMut<PlayerAlive>,
    mut player_query: Query<(Entity, &Transform, With<Player>)>,
    mut enemy_query: Query<(&Transform ,With<Enemy>)>
){
    if let Ok((entity, player_tf, _)) = player_query.single_mut() {
        if !alive.0 {return;}

        for (enemy_tf, _) in enemy_query.iter_mut() {
            let margin = 17.;

            let player_size = Vec2::new(PLAYER_SIZE.0*player_tf.scale[0].abs()-margin, PLAYER_SIZE.1*player_tf.scale[1].abs()-margin);
            let enemy_size = Vec2::new(ENEMY_SIZE.0*enemy_tf.scale[0].abs()-margin, ENEMY_SIZE.1*enemy_tf.scale[1].abs()-margin);
            let collision = collide(player_tf.translation, player_size, enemy_tf.translation, enemy_size);

            if let Some(_) = collision{
                alive.0 = false;
                commands.entity(entity).despawn();

                //Spawn dead player
                commands
                .spawn_bundle(SpriteSheetBundle  {
                    texture_atlas: materials.player_d.clone(),
                    transform: Transform{
                        translation: player_tf.translation,
                        scale: Vec3::new(0.30, 0.30, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(DeadPlayer)
                .insert(AffectedByGravity(true))
                .insert(Timer::from_seconds(0.05, true))
                .insert(Animation{index: 0,size: 30})
                .insert(Velocity(Vec2::ZERO));
            }
        }
    }
}

fn reset_player_pos(
    win_size: Res<WinSize>,
    mut player_query: Query<(&mut Transform, &mut Velocity, With<Player>)>
){
    if let Ok((mut transform,mut velocity, _)) = player_query.single_mut() {

        let bottom = - win_size.h/2. + 160.0;
        let left = - win_size.w/2. + 135.0;
        transform.translation = Vec3::new(left, bottom, 10.);
        velocity.0 = Vec2::ZERO;
    }
}

//Player Dead

fn player_dead_movement(
    win_size: Res<WinSize>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut AffectedByGravity, With<DeadPlayer>)>,
){
    if let Ok((mut transform, mut velocity, mut in_air, _)) = player_query.single_mut() {
        transform.translation.y += velocity.0.y * TIME_STEP;
        transform.translation.x += velocity.0.x * TIME_STEP;

        if transform.translation.y <  (- win_size.h/2. + 160.0) {
            velocity.0.y = 0.;
            transform.translation.y = - win_size.h/2. + 160.0;
            in_air.0 = false;
        }
    }
}

fn animate_dead_player(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Animation, With<DeadPlayer>)>,
) {
    for (mut timer, mut sprite, anim, _) in query.iter_mut() {
        if sprite.index+1 >= anim.index+anim.size {
            match game_state.set(GameState::Dead) {
                Ok(_) => {println!("Dead State");}
                Err(_) => {}
            }
            return;
        }
        
        timer.tick(time.delta());
        if timer.finished() {
            if sprite.index >= anim.index && sprite.index+1 < anim.index+anim.size {
                sprite.index += 1;
            } else {
                sprite.index = anim.index;
            }
        }
    }
}

fn player_dead_cleanup(
    mut commands: Commands,
    mut alive: ResMut<PlayerAlive>,
    mut query: Query<(Entity, With<DeadPlayer>)>,
){
    if let Ok((entity, _)) = query.single_mut(){
        alive.0 = true;
        commands.entity(entity).despawn();
    }
}
