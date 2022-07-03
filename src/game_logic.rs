use bevy::{prelude::*, utils::{HashMap}, render::camera::{ScalingMode}, sprite::Anchor};
use crate::{game_logic_types::*, AppState, LevelIndex};

pub fn destroy_blocks(mut commands: Commands, blocks: Query<Entity, With<Block>>, cameras: Query<Entity, With<Camera>>) {
    for entity_id in blocks.iter() {
        commands.entity(entity_id).despawn_recursive();
    }

    for entity_id in cameras.iter() {
        commands.entity(entity_id).despawn_recursive();
    }
}

pub fn setup_images(asset_server: Res<AssetServer>, mut textures: ResMut<Textures>) {
    macro_rules! asset_to_block {
        ($block_type: expr, $asset_name: expr) => {
            let _image = asset_server.load($asset_name);
            textures.0.insert(($block_type, None), _image);
        };

        (text, $text_block_type: expr, $asset_name: expr) => {
            let _image = asset_server.load($asset_name);
            textures.0.insert((Block::Text, Some($text_block_type)), _image);
        };
    }

    asset_to_block!(Block::Bevy, "textures/bevy.png");
    asset_to_block!(Block::Wall, "textures/wall.png");
    asset_to_block!(Block::Rock, "textures/rock.png");
    asset_to_block!(Block::Flag, "textures/flag.png");

    asset_to_block!(text, TextBlock::Bevy, "textures/text bevy.png");
    asset_to_block!(text, TextBlock::Rock, "textures/text rock.png");
    asset_to_block!(text, TextBlock::Wall, "textures/text wall.png");
    asset_to_block!(text, TextBlock::You, "textures/text you.png");
    asset_to_block!(text, TextBlock::Push, "textures/text push.png");
    asset_to_block!(text, TextBlock::Stop, "textures/text stop.png");
    asset_to_block!(text, TextBlock::Is, "textures/text is.png");
    asset_to_block!(text, TextBlock::Flag, "textures/text flag.png");
    asset_to_block!(text, TextBlock::Win, "textures/text win.png");
}

pub fn setup_world(mut commands: Commands, textures: Res<Textures>, level_index: Res<LevelIndex>, levels: Res<Levels>, mut constraints: ResMut<Constraints>) {
    let levels = &levels.0;

    const CAMERA_WIDTH: f32 = 640.0;
    const CAMERA_HEIGHT: f32 = 360.0;

    commands.spawn().insert_bundle({
        let mut cam = OrthographicCameraBundle::new_2d();
        let ortho = &mut cam.orthographic_projection;

        ortho.near = -10.0;

        ortho.scaling_mode = ScalingMode::None;
        ortho.left = 0.0;
        ortho.bottom = 0.0;
        ortho.right = CAMERA_WIDTH;
        ortho.top = CAMERA_HEIGHT;
        
        cam
    });

    let level = &levels[level_index.0];
    
    let size = Vec2::new(level.width as f32 - 16.0, level.height as f32 - 16.0);
    let offset = Vec2::new(-(size.x - CAMERA_WIDTH) / 2.0, -(size.y - CAMERA_HEIGHT) / 2.0);

    constraints.start = offset;
    constraints.end = size + offset;

    for (block, pos) in &level.blocks {
        spawn_block(&mut commands, &textures, *block, *pos + offset);
    }

    for (text_block, pos) in &level.text_blocks {
        spawn_text_block(&mut commands, &textures, *text_block, *pos + offset);
    }

    let black_background = SpriteBundle {
        sprite: Sprite { 
            color: Color::BLACK, 
            custom_size: Some(size + Vec2::new(16.0, 16.0)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        transform: Transform::from_translation(offset.extend(-0.01) - Vec3::new(8.0, 8.0, 0.0)),
        ..default()
    };

    commands.spawn_bundle(black_background);
}

pub fn apply_constraints(mut movers: Query<(&mut Mover, &mut Transform), Changed<Mover>>, constraints: Res<Constraints>) {
    for (mut mover, mut transform) in movers.iter_mut() {
        if mover.complete { continue; }

        let target_pos = mover.target;
        let clamped_pos = target_pos.clamp(constraints.start.extend(0.0), constraints.end.extend(0.0));

        if target_pos != clamped_pos {
            transform.translation = clamped_pos;
            mover.complete = true;
        }
    }
}

pub fn check_if_win(mut commands: Commands, winners: Query<Entity, With<PlayerHasWon>>, mut app_state: ResMut<State<AppState>>) {
    for winner_id in winners.iter() {
        let _ = app_state.set(AppState::MainMenu);

        commands.entity(winner_id).despawn();
    }
}

pub fn apply_record_to_world(mut blocks: Query<(Entity, &mut Mover, &mut Sprite, &mut Block)>, world_recorder: Res<WorldRecorder>) {
    // we don't want to run this function if the world is being recorded. The reason is we wouldn't want the changes we do going back
    // causing it to become changes piled up on the recording.
    if world_recorder.user_input || world_recorder.record {
        return;
    }

    let record = match world_recorder.get() {
        Some(value) => value,
        None => return,
    };

    for data in record {
        for (id, mut mover, mut sprite, mut block) in blocks.iter_mut() {
            if data.id == id {
                mover.complete = false;
                mover.target = data.pos;
                sprite.flip_x = data.flip_sprite;
                *block = data.block_type;
            }
        }
    }
}

pub fn record_world(blocks: Query<(Entity, &Transform, &Sprite, &Block)>, mut world_recorder: ResMut<WorldRecorder>) {
    // should not start recording until user input is detected.
    if !world_recorder.user_input || !world_recorder.record {
        return;
    }

    for (id, transform, sprite, block) in blocks.iter() {
        world_recorder.push(id, transform.translation, *block, sprite.flip_x);
    }

    world_recorder.record = false;
}

pub fn evaluate_text(mover: Query<&Mover, Changed<Mover>>, text: Query<(&TextBlock, &Transform)>, mut block_attributes: ResMut<BlockAttributes>, mut queue: ResMut<Queue>) {
    if mover.is_empty() { return; }
    
    block_attributes.0.clear();
    block_attributes.0.insert(Block::Text, vec![Attribute::Push]);

    let mut vector_of_text = Vec::new();

    for (text_type, transform) in text.iter() {
        vector_of_text.push((*text_type, transform.translation));
    }

    fn get(vector_of_text: &Vec<(TextBlock, Vec3)>, position: Vec3) -> Option<TextBlock> {
        for (text_block, pos) in vector_of_text {
            if *pos == position {
                return Some(*text_block);
            }
        }

        None
    }
    
    let mut text_attributes = HashMap::<TextBlock, Vec<TextBlock>>::new();

    for (text_block, pos) in &vector_of_text {
        match text_block {
            TextBlock::Bevy | TextBlock::Wall | TextBlock::Rock | TextBlock::Flag => {
                let right_pos = *pos + Vec3::new(16.0, 0.0, 0.0);
                let down_pos = *pos + Vec3::new(0.0, -16.0, 0.0);

                // 0 => right, 1 => down
                let text_is_tile = {
                    let right = get(&vector_of_text, right_pos);
                    let down = get(&vector_of_text, down_pos);

                    let mut is_on_direction = [false, false];
                    let directions = [right, down];

                    let mut index = 0;
                    for direction in directions {
                        if direction.is_some() {
                            let is_tile = direction.unwrap();

                            match is_tile {
                                TextBlock::Is => is_on_direction[index] = true,
                                _ => {},
                            }
                        }
                        index += 1;
                    }

                    is_on_direction
                }; 

                let mut index = 0;
                for is_tile in text_is_tile {
                    if is_tile {
                        let pos = if index == 0 {
                            right_pos + Vec3::new(16.0, 0.0, 0.0)
                        } else {
                            down_pos + Vec3::new(0.0, -16.0, 0.0)
                        };

                        let tile = get(&vector_of_text, pos);

                        if tile.is_some() {
                            let tile = tile.unwrap();

                            match tile {
                                TextBlock::Is => {},
                                _ => {
                                    let attributes = text_attributes.get(text_block);

                                    let mut attributes = if attributes.is_none() {
                                        Vec::new()
                                    } else {
                                        (*attributes.unwrap()).clone()
                                    };

                                    attributes.push(tile);
                                    text_attributes.insert(*text_block, attributes);
                                }
                            }
                        }
                    }

                    index += 1
                }
            }

            _ => {},
        }
    }

    for (key, text_attributes) in text_attributes.into_iter() {
        let block_type = match key {
            TextBlock::Bevy => Block::Bevy,
            TextBlock::Rock => Block::Rock,
            TextBlock::Wall => Block::Wall,
            TextBlock::Flag => Block::Flag,

            _ => continue,
        };

        let mut attributes = Vec::<Attribute>::new();

        macro_rules! change_block {
            ($block: expr) => {
                {
                    queue.push(Entity::from_raw(0), QueueType::ChangeBlock(block_type, $block));
                    continue 
                }
            };
        }

        for text_attribute in text_attributes {
            attributes.push(match text_attribute {
                TextBlock::Push => Attribute::Push,
                TextBlock::Stop => Attribute::Stop,
                TextBlock::You => Attribute::You,
                TextBlock::Win => Attribute::Win,

                TextBlock::Bevy => change_block!(Block::Bevy),
                TextBlock::Rock => change_block!(Block::Rock),
                TextBlock::Wall => change_block!(Block::Wall),
                TextBlock::Flag => change_block!(Block::Flag),

                TextBlock::Is => continue,
            })
        }

        block_attributes.0.insert(block_type, attributes);
    }
}

macro_rules! unwrap_attributes {
    ($block_attributes: expr, $block: expr, $none_case: expr) => {
        {
            let attributes = $block_attributes.0.get(&$block);

            if attributes.is_none() { $none_case; }

            attributes.unwrap()
        }
    };
}

pub fn apply_queue(mut commands: Commands, mut blocks: Query<(Entity, &mut Block, &mut Mover)>, queue: Res<Queue>, tile_map: Res<TileMap>, block_attributes: Res<BlockAttributes>, constraints: Res<Constraints>) {
    if !queue.user_input {
        return;
    }
   
    let mut moveables = Vec::<(Entity, Vec3)>::new();
    let mut transform_types = HashMap::<Block, Block>::new();
    
    for entry in queue.clone() {
        match entry.queue_type {
            QueueType::Delete => {
                commands.entity(entry.id).despawn_recursive();
            }

            QueueType::WinOn(pos) => {
                let tile = tile_map.get(pos);
                if let Some(tuples) = tile {
                    for tuple in tuples {
                        let (_, _, block) = tuple;

                        let attributes = unwrap_attributes!(block_attributes, block, continue);

                        for attribute in attributes {
                            match attribute {
                                Attribute::You => {
                                    commands.spawn().insert(PlayerHasWon);
                                }

                                _ => {},
                            }
                        }
                    }
                }
            }

            QueueType::ChangeBlock(from, to) => {
                transform_types.insert(from, to);
            }

            QueueType::Move(direction, position) => {
                let mut pushables = Vec::<(Entity, Vec3)>::new();

                let increment = match direction {
                    BlockDirection::Down  => Vec3::new(0.0, -16.0, 0.0),
                    BlockDirection::Left  => Vec3::new(-16.0, 0.0, 0.0),
                    BlockDirection::Right => Vec3::new(16.0, 0.0, 0.0),
                    BlockDirection::Up    => Vec3::new(0.0, 16.0, 0.0),

                    BlockDirection::None => {
                        pushables.push((entry.id, position));
                        moveables.append(&mut pushables);
                        continue;
                    },
                };

                pushables.push((entry.id, position + increment));

                fn check_for_pushable_tiles(pushables: &mut Vec<(Entity, Vec3)>, tile_map: &TileMap, block_attributes: &BlockAttributes, constraints: &Constraints, position: Vec3, increment: Vec3) {
                    let position = position + increment;

                    {
                        let clamped_position = position.truncate().clamp(constraints.start, constraints.end);

                        if position.truncate() != clamped_position {
                            pushables.clear();
                            return;
                        }
                    }

                    let tile = tile_map.get(position);

                    if let Some(tuples) = tile {
                        for tuple in tuples {
                            let (id, _, block) = tuple;

                            let attributes = unwrap_attributes!(block_attributes, block, continue);
                            
                            for attribute in attributes {
                                match attribute {
                                    Attribute::Stop => pushables.clear(),
                                    Attribute::Push => {
                                        pushables.push((id, position + increment));
                                        check_for_pushable_tiles(pushables, tile_map, block_attributes, constraints, position, increment);
                                    }

                                    _ => continue,
                                }
                            }
                        }
                    }
                }

                check_for_pushable_tiles(&mut pushables, &tile_map, &block_attributes, &constraints, position, increment);

                moveables.append(&mut pushables);
            }
        }
    }

    // applys all movement
    for (entity_id, mut block_id, mut mover) in blocks.iter_mut() {
        let transform_to = transform_types.get(&block_id);

        if let Some(transform_to) = transform_to {
            *block_id = *transform_to;
        }

        let mut found_matching_id: Option<usize> = None;
        for (index, (id, pos)) in moveables.iter().enumerate() {
            if entity_id.eq(id) {
                found_matching_id = Some(index);

                if !mover.complete { break; }

                mover.complete = false;
                mover.target = *pos;
            }
        }

        if found_matching_id.is_some() {
            moveables.swap_remove(found_matching_id.unwrap());
        }
    }
}

pub fn map_tiles(movers: Query<&Mover, Changed<Mover>>, blocks: Query<(&Transform, &Block, Entity)>, mut tile_map: ResMut<TileMap>) {
    if movers.is_empty() { return; } 

    let mut is_done_moving = true;
    movers.for_each(|mover| {
        is_done_moving = is_done_moving && mover.complete;
    });

    if !is_done_moving { return; }

    tile_map.clear();

    blocks.for_each(|(transform, block_id, entity_id)| {
        let pos = transform.translation;
        tile_map.push((entity_id, pos, *block_id));
    });
}

pub fn apply_mover(mut blocks: Query<(&mut Mover, &mut Transform)>, timer: Res<Time>) {
    blocks.for_each_mut(|(mut mover, mut transform)| {
        if mover.complete { return; }
        
        transform.translation = transform.translation.lerp(mover.target, 19.5 * timer.delta_seconds());

        if transform.translation.round() == mover.target {
            mover.complete = true;
            transform.translation = transform.translation.round();
        }
    })
}

pub fn apply_attributes(
    movers: Query<&Mover>, 
    mut blocks: Query<(Entity, &Block, &Transform, &mut Sprite)>, 
    mut queue: ResMut<Queue>, 
    mut world_recorder: ResMut<WorldRecorder>,
    block_attributes: Res<BlockAttributes>, 
    keys: Res<Input<KeyCode>>) 
{
    {
        let mut logic_continue = true;
        for mover in movers.iter() {
            logic_continue = mover.complete && logic_continue;
        }

        if !logic_continue { return; }
    }
    
    queue.reset();
    world_recorder.reset();

    if keys.pressed(KeyCode::R) {
        world_recorder.redo();
        return;
    }

    if keys.pressed(KeyCode::T) {
        world_recorder.undo();
        return;
    }

    blocks.for_each_mut(|(entity_id, block, transform, mut sprite)| {
        let attributes = unwrap_attributes!(block_attributes, *block, return);

        for attribute in attributes {
            match attribute {
                Attribute::You => {
                    let mut current_direction = BlockDirection::None;

                    if keys.pressed(KeyCode::A) {
                        current_direction = BlockDirection::Left;
                        sprite.flip_x = true;
                    }

                    if keys.pressed(KeyCode::D) {
                        current_direction = BlockDirection::Right;
                        sprite.flip_x = false;
                    }

                    if keys.pressed(KeyCode::W) {
                        current_direction = BlockDirection::Up;
                    }

                    if keys.pressed(KeyCode::S) {
                        current_direction = BlockDirection::Down;
                    }

                    if current_direction == BlockDirection::None {
                        return;
                    }

                    world_recorder.user_input = true;
                    queue.user_input = true;

                    queue.push(entity_id, QueueType::Move(current_direction, transform.translation));
                },

                Attribute::Win => {
                    queue.push(entity_id, QueueType::WinOn(transform.translation));
                }

                _ => {}
            }
        }
    })
}

pub fn change_block_texture(mut blocks: Query<(&mut Handle<Image>, &Block), Changed<Block>>, textures: Res<Textures>) {
    blocks.for_each_mut(|(mut image, block)| {
        match block {
            Block::Text => {},
            _ => *image = block_to_texture(&textures, *block, None)
        }
    });
}

fn block_to_texture(textures: &Res<Textures>, block: Block, optional_text: Option<TextBlock>) -> Handle<Image> {
    (*textures.0.get(&(block, optional_text)).unwrap()).clone()
}

fn spawn_text_block(commands: &mut Commands, textures: &Res<Textures>, text_type: TextBlock, tile_pos: Vec2) {
    commands.spawn()
        .insert_bundle(TextBlockBundle {
            type_id: Block::Text,
            text_type,
            mover: Mover {
                target: default(),
                complete: true,
            }
        })
        .insert_bundle(SpriteBundle {
            texture: (*(textures.0.get(&(Block::Text, Some(text_type))).unwrap())).clone(),
            transform: Transform::from_translation(Vec3::new(tile_pos.x, tile_pos.y, 0.0)),
            ..default()
        });
}

fn spawn_block(commands: &mut Commands, textures: &Res<Textures>, type_id: Block, tile_pos: Vec2) {
    commands.spawn()
        .insert_bundle(BlockBundle {
            type_id,
            mover: Mover { 
                target: default(), 
                complete: true
            }
        })
        .insert_bundle(SpriteBundle {
            texture: block_to_texture(textures, type_id, None),
            transform: Transform::from_translation(Vec3::new(tile_pos.x, tile_pos.y, 0.0)),
            ..default()
        });
}