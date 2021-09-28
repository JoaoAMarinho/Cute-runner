use bevy::prelude::*;
use player::*;
use gamestate::{GameState};

use crate::{Materials};
use crate::player;
use crate::gamestate;

// region:    Components
struct Score(f32);
// endregion:    Components

// region:    Plugin
pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(menu_text_spawn.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(menu_text_cleanup.system())
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(score_spawn.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_score.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Dead)
                    .with_system(score_cleanup.system())
            );
    }
}
// endregion:    Plugin

fn score_spawn(
    mut commands: Commands,
    materials: Res<Materials>
) {
    //Score text
    commands.
        spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Score: ",
                TextStyle {
                    font: materials.font.clone(),
                    font_size: 50.0,
                    color: Color::rgb(0.0823, 0.0627, 0.1686),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(-380.,190.,30.),
                ..Default::default()
            },
            ..Default::default()
        });
    //Score value
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: materials.font.clone(),
                    font_size: 50.0,
                    color: Color::rgb(0.0823, 0.0627, 0.1686),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(-255.,188.,30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score(0.));
}

fn update_score(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Score)>,
    player_alive: Res<PlayerAlive>
){
    
    if !player_alive.0 {return;}
    
    if let Ok((mut text, mut score)) = query.single_mut() {
        score.0 += time.delta_seconds();
        let value = score.0 as u32;
        let string = format!("{:05}", value);

        text.sections[0].value = string;
    }

}

fn score_cleanup(
    mut commands: Commands,
    mut query: Query<(Entity, &Text)>
){
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn menu_text_spawn(
    mut commands: Commands,
    materials: Res<Materials>
){
    //Press enter text
    commands.
        spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "<Press Enter>",
                TextStyle {
                    font: materials.font.clone(),
                    font_size: 80.0,
                    color: Color::rgb(0.0823, 0.0627, 0.1686),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(0.,50.,30.),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn menu_text_cleanup(
    mut commands: Commands,
    mut text_query: Query<(Entity, With<Text>)>
){
    if let Ok((entity, _)) = text_query.single_mut() {
        commands.entity(entity).despawn();
    }
}