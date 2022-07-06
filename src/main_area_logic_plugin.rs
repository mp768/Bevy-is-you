use bevy::prelude::*;
use crate::{AppState, game_logic::*, LevelIndex, loading_levels::*, load_levels, game_logic_types::*};

fn change_level_to_main(mut level_index: ResMut<LevelIndex>) {
    level_index.0 = 0;
}

pub struct MainAreaPlugin;

impl Plugin for MainAreaPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(load_levels!("main map", "_test_level", "actual_puzzle"))
            .init_resource::<Textures>()
            .init_resource::<BlockAttributes>()
            .init_resource::<TileMap>()
            .init_resource::<Queue>()
            .init_resource::<Constraints>()
            .init_resource::<WorldRecorder>()
            .add_startup_system(setup_images)
            .add_system_set(
                SystemSet::on_enter(AppState::MainArea)
                    .with_system(change_level_to_main)
                    .with_system(setup_world.after(change_level_to_main))
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainArea)
                    .with_system(apply_mover)
                    .with_system(change_block_texture)
                    .with_system(apply_constraints.before(apply_mover))
                    .with_system(map_tiles.after(apply_mover))
                    .with_system(record_world.after(apply_attributes).before(apply_queue))
                    .with_system(apply_attributes.after(map_tiles))
                    .with_system(apply_queue.after(apply_attributes))
                    .with_system(evaluate_text.after(apply_attributes).before(apply_queue))
                    .with_system(apply_record_to_world.after(apply_attributes).before(apply_queue).after(record_world))
                    .with_system(check_if_level_changed)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainArea)
                    .with_system(destroy_sprites)
            );
    }
}