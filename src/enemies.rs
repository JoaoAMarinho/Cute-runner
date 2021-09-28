use bevy::prelude::*;
use physics::*;
use player::*;
use gamestate::{GameState};

use rand::{thread_rng, Rng};

use crate::{Materials, TIME_STEP};
use crate::{Animation, LoopAnim};
use crate::physics;
use crate::player;
use crate::gamestate;

// region:    Resources
struct SpawnTimer {
    pub timer: Timer,
    //Time to add to spawn
    difficulty: f32
}

struct EnemySpawnSettings {
    pub min_time: f32,
    pub max_time: f32,
    pub speed: f32,
}
// endregion:    Resources

// region:    Components
pub struct Enemy;
// endregion:    Components

// region:    Plugin
pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(reset_spawn_timer.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(enemy_spawn.system())
                    .with_system(enemies_movement.system())
                    .with_system(enemies_offscreen.system())
                    .with_system(difficulty_setter.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Dead)
                    .with_system(enemies_movement.system())
                    .with_system(enemies_offscreen.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Dead)
                    .with_system(enemies_cleanup.system())
                    
            )
            .insert_resource( SpawnTimer {
                timer: Timer::from_seconds(2.5, true),
                difficulty: 3.
            })
            .insert_resource( EnemySpawnSettings {
                min_time: 1.5,
                max_time: 4.,
                speed: -170.0
            });
    }
}
// endregion:    Plugin

fn enemy_spawn(
    mut commands: Commands,
    enemy_settings: Res<EnemySpawnSettings>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    materials: Res<Materials>,
    player_alive: Res<PlayerAlive>
){
    if !player_alive.0 {return;}
    
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.finished() {
        return;
    }

    let mut rng = thread_rng();
    let random_time = rng.gen_range(enemy_settings.min_time..enemy_settings.max_time);
    spawn_timer.timer = Timer::from_seconds(random_time+spawn_timer.difficulty, true);

    let height = rng.gen_range(0.0..1.5) as i32;
    
    commands
        .spawn_bundle(SpriteSheetBundle  {
            texture_atlas: materials.enemies.clone(),
            transform: Transform{
                translation: Vec3::new( 500., -100.+ 135.* height as f32, 15.),
                scale: Vec3::new(0.20, 0.20, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Timer::from_seconds(0.05, true))
        .insert(Animation{index: 0,size: 13})
        .insert(LoopAnim)
        .insert(Velocity(Vec2::new(enemy_settings.speed, 0.)));
}

fn enemies_movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform, With<Enemy>)>
){
    for (velocity, mut transform, _) in query.iter_mut() {
        let base: f32 = 10.;
        let delta_time = time.time_since_startup().as_secs() as f32 + time.time_since_startup().subsec_nanos() as f32 * f32::powi(base, -9);
        
        transform.translation.x += velocity.0.x * TIME_STEP;
        let amplitude = 0.5;
        let frequency = 2.;
        transform.translation.y +=  amplitude * (frequency * delta_time).sin();
    }
}

fn enemies_offscreen(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, With<Enemy>)>
){
    for (entity, enemy_tf, _) in enemy_query.iter_mut() {
        if enemy_tf.translation.x <= -550. {
                commands.entity(entity).despawn();
        }
    }
}

fn difficulty_setter(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>
){
    spawn_timer.difficulty -= time.delta_seconds() as f32 * 0.05;
    spawn_timer.difficulty = spawn_timer.difficulty.max(0.);
}

fn reset_spawn_timer(
    mut spawn_timer: ResMut<SpawnTimer>
){
    spawn_timer.difficulty = 3.;
    spawn_timer.timer = Timer::from_seconds(2.5, true);
}

fn enemies_cleanup(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, With<Enemy>)>
){
    for (entity, _) in enemy_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}