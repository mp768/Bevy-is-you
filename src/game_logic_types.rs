use bevy::{prelude::*, utils::{HashMap}};

#[derive(Component)]
pub struct PlayerHasWon;

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


#[derive(Default)]
pub struct Constraints {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Block {
    Bevy,
    Wall,
    Rock,
    Text,
    Flag,
    Tree,
    Path,
    Water,
    Level01,
    Level02,
    Level03,
    Level04,
    Level05,
    Level06,
    Level07,
    Level08,
    Level09,
    Air,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TextBlock {
    Bevy,
    Is,
    You,
    Rock,
    Push,
    Wall,
    Stop,
    Flag,
    Win,
    Tree,
    Sink,
    Water,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockDirection {
    Right,
    Left,
    Up,
    Down,
    None,
}

#[derive(Component)]
pub struct Mover {
    pub target: Vec3,
    pub complete: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Attribute {
    You,
    Stop,
    Push,
    Win,
    Sink,
}

#[derive(Bundle)]
pub struct BlockBundle {
    pub type_id: Block,
    pub mover: Mover,
}

#[derive(Bundle)]
pub struct TextBlockBundle {
    pub type_id: Block,
    pub text_type: TextBlock,
    pub mover: Mover,
}

#[derive(Clone, Copy)]
pub struct RecordData {
    pub id: Entity,
    pub pos: Vec3,
    pub block_type: Block,
    pub flip_sprite: bool,
}

// records every manipulatable attribute of every entity on the screen after user input
#[derive(Default)]
pub struct WorldRecorder {
    pub records: Vec<Vec<RecordData>>,
    pub head_len: isize,
    pub user_input: bool,
    pub record: bool,
}

impl WorldRecorder {
    pub fn reset(&mut self) {
        if self.user_input {
            self.records.resize(self.head_len as usize, Vec::new());
            self.head_len += 1;
            self.records.push(Vec::new());
        }

        self.record = true;
        self.user_input = false;
    }

    pub fn get(&self) -> Option<&Vec<RecordData>> {
        self.records.get((self.head_len-1) as usize)
    }

    pub fn push(&mut self, id: Entity, pos: Vec3, block_type: Block, flip_sprite: bool) {
        self.records.resize(self.head_len as usize, Vec::new());

        let record = match self.records.get_mut((self.head_len-1) as usize) {
            Some(value) => value,
            None => {
                self.records.push(Vec::new());
                self.head_len = self.records.len() as isize;

                &mut self.records[(self.head_len-1) as usize]
            }
        };

        record.push(RecordData {
            id,
            pos,
            block_type,
            flip_sprite,
        });
    }

    pub fn undo(&mut self) {
        self.record = false;
        self.head_len -= 1;

        if self.head_len <= 0 {
            self.head_len = 0;
        }
    }

    pub fn redo(&mut self) {
        self.record = false;
        self.head_len += 1;

        if self.head_len as usize >= self.records.len() {
            self.head_len = self.records.len() as isize;
        }
    }
}

#[derive(Default)]
pub struct Textures(pub HashMap<(Block, Option<TextBlock>), Handle<Image>>);

#[derive(Default)]
pub struct BlockAttributes(pub HashMap<Block, Vec<Attribute>>);

#[derive(Clone, Copy, Debug)]
pub enum QueueType {
    Move(BlockDirection, Vec3),
    ChangeBlock(Block, Block),
    WinOn(Vec3),
    Sink(Vec3),
    Delete,
}

#[derive(Clone, Copy, Debug)]
pub struct QueueEntry {
    pub id: Entity,
    pub queue_type: QueueType,
}

#[derive(Default, Clone, Debug)]
pub struct Queue {
    entries: Vec<QueueEntry>,
    iter_idx: usize,
}

impl Queue {
    pub fn push(&mut self, entity_id: Entity, queue_type: QueueType) {
        for entry in &self.entries {
            if entity_id == entry.id {
                return;
            }
        }

        self.entries.push(QueueEntry {
            id: entity_id,
            queue_type
        });
    }

    pub fn get(&self, index: usize) -> Option<QueueEntry> {
        match self.entries.get(index) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn reset(&mut self) {
        self.entries.clear();
    }
}

impl Iterator for Queue {
    type Item = QueueEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.get(self.iter_idx);
        self.iter_idx += 1;

        if self.iter_idx > self.entries.len() {
            self.iter_idx = 0;
        }

        tmp
    }
}

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

type TileInfo = (Entity, Vec3, Block);

#[derive(Default, Debug)]
pub struct TileMap {
    content: Vec<TileInfo>,
}

impl TileMap {
    pub fn get(&self, translation: Vec3) -> Option<Vec<TileInfo>> {
        let translation = translation.truncate();
        let mut entries = Vec::new();

        for item in &self.content {
            let (_, pos, _) = item;

            if pos.truncate() == translation {
                entries.push(item.clone());
            }
        }

        if entries.is_empty() {
            None
        } else {
            Some(entries)
        }
    }

    #[inline]
    pub fn push(&mut self, item: TileInfo) {
        self.content.push(item);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.content.clear();
    }
    
}