use bevy::prelude::*;

// region:    State
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Dead
}
// endregion:    State

// region:    Plugin
pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(GameState::MainMenu)
            .add_system(handle_gamestate.system());
    }
}
// endregion:    Plugin

fn handle_gamestate(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>
){
    match game_state.current() {
        GameState::MainMenu => {
            if keyboard_input.pressed(KeyCode::Return) {
                match game_state.set(GameState::Playing) {
                    Ok(_) => {println!("Playing State");}
                    Err(_) => {}
                }
            }
        }
        GameState::Playing => {
            //Create pause mode?
        }
        GameState::Dead => {
            if keyboard_input.pressed(KeyCode::Escape) {
                match game_state.set(GameState::Playing) {
                    Ok(_) => {println!("Playing State");}
                    Err(_) => {}
                }
            }
        }
    }
}
