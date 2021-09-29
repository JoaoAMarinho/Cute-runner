mod player;
mod physics;
mod enemies;
mod score;
mod gamestate;

use bevy::prelude::*;

use player::*;
use physics::*;
use enemies::*;
use score::*;
use gamestate::*;

// region:    Constants
const PLAYER_SPRITE_A: &str = "sprites/cute_girl_alive.png";
const PLAYER_SPRITE_D: &str = "sprites/cute_girl_dead.png";
const BACKGROUND_IMG: &str = "textures/background.png";
const ENEMIES: &str = "sprites/enemies_red.png";
//const ENEMIES: &str = "sprites/enemies_blue.png";
const CANDY_FONT: &str = "fonts/CandyshopRegular.otf";
const TIME_STEP: f32 = 1./60.;
const PLAYER_SIZE: (f32,f32) = (416., 454.);
const PLAYER_DEAD_SIZE: (f32,f32) = (601., 512.);
const ENEMY_SIZE: (f32,f32) = (273., 282.);
// endregion:    Constants

//Entity, Component, System, Resource

// region:    Resources
pub struct Materials {
    player_a: Handle<TextureAtlas>,
    player_d: Handle<TextureAtlas>,
    enemies: Handle<TextureAtlas>,
    font: Handle<Font>
}
pub struct Sounds {
    jump: Handle<AudioSource>
}
pub struct WinSize {
    w: f32,
    h: f32
}
// endregion:    Resources

// region:    Components
struct LoopAnim;
struct Animation{
    index: u32,
    size: u32
}
// endregion:    Components

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Rust Surviver!".to_string(),
            width: 1000.0,
            height: 555.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameStatePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemiesPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup.system())
        .add_system(animate_looping_sprites.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let window = windows.get_primary_mut().unwrap();
    
    //Spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    //Load textures
    let texture_handle_pa = asset_server.load(PLAYER_SPRITE_A);
    let texture_handle_pd = asset_server.load(PLAYER_SPRITE_D);
    
    let texture_atlas_player_d = TextureAtlas::from_grid(texture_handle_pd, Vec2::new(PLAYER_DEAD_SIZE.0, PLAYER_DEAD_SIZE.1), 30, 1);
    let texture_atlas_player_a = TextureAtlas::from_grid(texture_handle_pa, Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1), 33, 2);
    
    let texture_handle_e = asset_server.load(ENEMIES);
    let texture_atlas_e = TextureAtlas::from_grid(texture_handle_e, Vec2::new(ENEMY_SIZE.0, ENEMY_SIZE.1), 13, 1);

    //Create resources
    commands
        .insert_resource(Materials {
            player_a: texture_atlases.add(texture_atlas_player_a),
            player_d: texture_atlases.add(texture_atlas_player_d),
            enemies: texture_atlases.add(texture_atlas_e),
            font: asset_server.load(CANDY_FONT)
        });
    commands
        .insert_resource(WinSize {
            w: window.width(),
            h: window.height()
        });
    commands
        .insert_resource(Gravity(45.*25.));
    commands
        .insert_resource(Sounds{
            jump: asset_server.load("sounds/jump.mp3")
        });

    //Spawn background
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(BACKGROUND_IMG).into()),
        ..Default::default()
    });
}

fn animate_looping_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Animation, With<LoopAnim>)>,
) {
    for (mut timer, mut sprite, anim, _) in query.iter_mut() {
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
