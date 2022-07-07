use bevy::{prelude::*, utils::{HashMap}};

#[derive(Component)]
pub struct PlayerHasWon;

#[derive(Component)]
pub struct PlayerLevelSelect(pub usize);

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
    LevelSelect(usize),
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

    pub fn clear(&mut self) {
        self.record = false;
        self.head_len = 0;
        self.records.clear();
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
    LevelSelect(Vec3, usize),
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

    pub fn push_type(&mut self, queue_type: QueueType) {
        self.entries.push(QueueEntry { 
            id: Entity::from_raw(0), 
            queue_type, 
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