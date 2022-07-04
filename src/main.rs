pub mod game_logic;
pub mod game_logic_types;
pub mod game_logic_plugin;
pub mod menu_logic;
pub mod menu_logic_types;
pub mod menu_logic_plugin;

use bevy::{prelude::*};
use crate::{game_logic_plugin::*};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

pub struct LevelIndex(usize);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(51, 50, 60)))
        .insert_resource(WindowDescriptor {
            title: "Bevy is you".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        }) 
        .insert_resource(LevelIndex(1))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Game)
        .add_plugin(GameLogicPlugin)
        .run();
}

