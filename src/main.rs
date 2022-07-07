pub mod game_logic;
pub mod game_logic_types;
pub mod game_logic_plugin;
pub mod main_area_logic_plugin;
pub mod loading_levels;

use bevy::{prelude::*};
use main_area_logic_plugin::MainAreaPlugin;
use crate::{game_logic_plugin::*};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum AppState {
    MainArea,
    Game,
}

pub struct LevelIndex(pub usize);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(51, 50, 60)))
        .insert_resource(WindowDescriptor {
            title: "Bevy is you".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        }) 
        .insert_resource(LevelIndex(0))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainArea)
        .add_plugin(GameLogicPlugin)
        .add_plugin(MainAreaPlugin)
        .run();
}
