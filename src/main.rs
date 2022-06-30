pub mod game_logic;
pub mod game_logic_types;
pub mod game_logic_plugin;

use bevy::{prelude::*};
use crate::{game_logic_plugin::*};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

pub struct LevelIndex(usize);

fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn_bundle(UiCameraBundle::default());

    let node_bundle = NodeBundle {
        style: Style {
            size: Size { 
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0)
            },
            position: Rect {
                left: Val::Percent(10.0),
                ..default()
            },
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        color: UiColor::from(Color::BLACK),
        ..default()
    };

    let text_bundle = TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position: Rect {
                left: Val::Px(500.0),
                right: Val::Px(800.0),
                bottom: Val::Px(500.0),
                top: Val::Px(800.0),
            },
            ..default()
        },
        text: Text::with_section(
            "You've won!", 
            TextStyle { 
                font, 
                font_size: 150.0, 
                color: Color::rgba(1.0, 1.0, 1.0, 0.95) 
            }, 
            TextAlignment { 
                vertical: VerticalAlign::Center, 
                horizontal: HorizontalAlign::Center
            }
        ),
        ..default()
    };

    commands.spawn_bundle(node_bundle).with_children(|parent| {
        parent.spawn_bundle(text_bundle);
    });
}

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
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(draw_ui))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Game)
        .add_plugin(GameLogicPlugin)
        .run();
}

