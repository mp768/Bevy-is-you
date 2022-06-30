use bevy::prelude::*;

use crate::{AppState, game_logic::*, game_logic_types::*, load_levels};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Textures>()
            .init_resource::<BlockAttributes>()
            .init_resource::<TileMap>()
            .init_resource::<Queue>()
            .init_resource::<Constraints>()
            .insert_resource(load_levels!("_test_level", "actual_puzzle"))
            .add_startup_system(setup_images)
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_world)   
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(apply_mover)
                    .with_system(change_block_texture)
                    .with_system(apply_constraints.before(apply_mover))
                    .with_system(map_tiles.after(apply_mover))
                    .with_system(apply_attributes.after(map_tiles))
                    .with_system(apply_queue.after(apply_attributes))
                    .with_system(evaluate_text.after(apply_attributes).before(apply_queue))
                    .with_system(check_if_win)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Game)
                    .with_system(destroy_blocks)   
            );
    }

    fn name(&self) -> &str {
        "Game Logic"
    }
}