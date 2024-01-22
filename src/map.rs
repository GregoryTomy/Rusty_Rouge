use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize; // usize uses preferred bit size of the CPU

#[derive(Copy, Clone, PartialEq)]   // macro attribute to implement the Copy, Clone, and PartialEq trains
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize // vectors are indexed by usize
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
    // perform  bounds checking
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    // function checks if players can enter a tile both dimensionally and for TileType
    pub fn can_enter_tile(&self, point: Point) -> bool {
        let idx = map_index(point.x, point.y);
        self.in_bounds(point) && self.tiles[idx] == TileType::Floor
    }
    // determine a tile's index coordinates and indicate error if requested coordinates fall outside of map boundries
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }
}





























