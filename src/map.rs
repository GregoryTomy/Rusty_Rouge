use crate::prelude::*;
use bracket_lib::prelude as brac;

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
    // index mapping
    // function receives the borrowed Camera and uses the boundaries from it to
    // render just the visible part of the map
    pub fn render(&self, context: &mut brac::BTerm, camera: &Camera) {
        context.set_active_console(0);  // render the first console layer, the base map
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(brac::Point::new(x, y)) {
                    let idx = map_index(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            context.set(
                                x - camera.left_x,  // move tiles relative to camera
                                y - camera.top_y,
                                brac::YELLOW,
                                brac::BLACK,
                                brac::to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            context.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                brac::GOLD,
                                brac::BLACK,
                                brac::to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }
    // perform  bounds checking
    pub fn in_bounds(&self, point: brac::Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    // function checks if players can enter a tile both dimensionally and for TileType
    pub fn can_enter_tile(&self, point: brac::Point) -> bool {
        let idx = map_index(point.x, point.y);
        self.in_bounds(point) && self.tiles[idx] == TileType::Floor
    }
    // determine a tile's index coordinates and indicate error if requested coordinates fall outside of map boundries
    pub fn try_index(&self, point: brac::Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }
}





























