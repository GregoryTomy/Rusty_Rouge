use crate::prelude as main;

const NUM_TILES: usize = (main::SCREEN_WIDTH * main::SCREEN_HEIGHT) as usize; // usize uses preferred bit size of the CPU

#[derive(Copy, Clone, PartialEq)]   // macro attribute to implement the Copy, Clone, and PartialEq trains
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_indexing(x: i32, y: i32) -> usize {
    ((y * main::SCREEN_WIDTH) + x) as usize // vectors are indexed by usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, context: &mut main::brac::BTerm) {
        for y in 0..main::SCREEN_HEIGHT {
            for x in 0..main::SCREEN_WIDTH {
                let idx = map_indexing(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        context.set(x, y, main::brac::YELLOW, main::brac::BLACK, main::brac::to_cp437('.'));
                    }
                    TileType::Wall => {
                        context.set(x, y, main::brac::GREEN, main::brac::BLACK, main::brac::to_cp437('#'));
                    }
                }
            }
        }
    }
}





























