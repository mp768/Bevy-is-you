use bevy::prelude::*;

use crate::{AppState, game_logic::*};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
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
                    .with_system(record_world.after(apply_attributes).before(apply_queue))
                    .with_system(apply_attributes.after(map_tiles))
                    .with_system(apply_queue.after(apply_attributes))
                    .with_system(evaluate_text.after(apply_attributes).before(apply_queue))
                    .with_system(apply_record_to_world.after(apply_attributes).before(apply_queue).after(record_world))
                    .with_system(check_if_win)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Game)
                    .with_system(destroy_sprites)   
            );
    }

    fn name(&self) -> &str {
        "Game Logic"
    }
}