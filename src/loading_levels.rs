
use bevy::prelude::*;
use crate::game_logic_types::*;

#[macro_export]
macro_rules! load_levels {
    ($($level_name: expr),*) => {{
        let mut levels = Vec::<LevelData>::new();

        $(
            {
                let level_string_data = include_str!(concat!("../assets/maps/", $level_name, ".json"));
                let level_json: serde_json::Value = match serde_json::from_str(level_string_data) {
                    Ok(value) => value,
                    Err(err) => {
                        panic!("Failed to load \"{}\": {}", $level_name, err);
                    }
                };

                let level_data = create_level_data(level_json);

                levels.push(level_data);
            }
        )*

        Levels(levels)
    }};
}

#[derive(Default)]
pub struct LevelData {
    pub width: usize,
    pub height: usize,
    pub blocks: Vec<(Block, Vec2)>,
    pub text_blocks: Vec<(TextBlock, Vec2)>,
}

pub struct Levels(pub Vec<LevelData>);

pub fn create_level_data(value: serde_json::Value) -> LevelData {
    let mut level_data: LevelData = LevelData {
        width: 0,
        height: 0,
        blocks: Vec::new(),
        text_blocks: Vec::new(),
    };

    level_data.width = value["width"].as_u64().unwrap() as usize;
    level_data.height = value["height"].as_u64().unwrap() as usize;
    
    let layer = value["layers"].as_array().unwrap();
    
    let width = (level_data.width / 16) as i32;
    let height = (level_data.height / 16) as i32;
    
    let mut current_row: i32;
    let mut current_column: i32;

    macro_rules! init {
        ($block_type: expr) => {{
            level_data.blocks.push(($block_type, Vec2::new((current_row * 16) as f32, (current_column * 16) as f32)));
        }};

        (text, $text_block_type: expr) => {{
            level_data.text_blocks.push(($text_block_type, Vec2::new((current_row * 16) as f32, (current_column * 16) as f32)));
        }};
    }

    macro_rules! data_loop {
        ($index: expr, { $($pat: pat => $result: expr),* }) => {
            current_row = 0;
            current_column = height-1;

            for data in layer[$index]["data"].as_array().unwrap() {
                match data.as_i64().unwrap() {
                    $(
                        $pat => {
                            $result
                        }
                    ),*
                }

                current_row += 1;

                if current_row >= width {
                    current_column -= 1;
                    current_row = 0;
                }
            }
        };
    }

    data_loop!(0, {
        0 => init!(Block::Bevy),
        1 => init!(Block::Wall),
        2 => init!(Block::Rock),
        3 => init!(Block::Flag),
        4 => init!(Block::Tree),
        5 => init!(Block::Level01),
        6 => init!(Block::Level02),
        7 => init!(Block::Level03),
        8 => init!(Block::Level04),
        9 => init!(Block::Level05),
        10 => init!(Block::Level06),
        11 => init!(Block::Level07),
        12 => init!(Block::Level08),
        13 => init!(Block::Level09),
        14 => init!(Block::Path),
        15 => init!(Block::Water),
        _ => {}
    });

    data_loop!(1, {
        0 => init!(text, TextBlock::Is),
        1 => init!(text, TextBlock::Bevy),
        2 => init!(text, TextBlock::You),
        3 => init!(text, TextBlock::Stop),
        4 => init!(text, TextBlock::Push),
        5 => init!(text, TextBlock::Wall),
        6 => init!(text, TextBlock::Rock),
        7 => init!(text, TextBlock::Flag),
        8 => init!(text, TextBlock::Win),
        9 => init!(text, TextBlock::Sink),
        10 => init!(text, TextBlock::Tree),
        11 => init!(text, TextBlock::Water),
        _ => {}
    });

    level_data
}