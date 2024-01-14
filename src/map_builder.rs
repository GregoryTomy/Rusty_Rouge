/*
room carving algorithm that starts with a solid block of walls and carves out
rooms and corrridors.
*/
use crate::prelude as main;
use crate::map;
use bracket_lib::prelude as brac;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: map::Map,
    // struct will have its own copy of map
    pub rooms: Vec<brac::Rect>,
    //  rooms will be added to the map
    pub player_start: brac::Point, // location at which the player starts the map
}

impl MapBuilder {
    // function obtains a mutable iterator and then changes every tile into a wall
    // |t| *t = tile is a closure. The iterator passes t as a reference and *t dereferences it
    fn fill(&mut self, tile: map::TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut brac::RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = brac::Rect::with_size(
                rng.range(1, main::SCREEN_WIDTH - 10),
                rng.range(1, main::SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10)
            );

            //  test the room against each placed room and flag if room intersects
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                // if the rooms don't overlap, check they are within map boundaries
                room.for_each(
                    |p| if p.x > 0 && p.x < main::SCREEN_WIDTH && p.y > 0 && p.y < main::SCREEN_HEIGHT
                    {
                        let index = map::map_index(p.x, p.y);
                        self.map.tiles[index] = map::TileType::Floor;
                    }
                );

                self.rooms.push(room)
            }
        }
    }

    // the hallways will use "dog-leg" corridors and will consist of a horizontal
    // and vertical section joined by a single corner
    fn build_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_index(brac::Point::new(x, y)) {
                self.map.tiles[index] = map::TileType::Floor;
            }
        }
    }

    fn build_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_index(brac::Point::new(x, y)) {
                self.map.tiles[index] = map::TileType::Floor;
            }
        }
    }

    // this function uses the horziontal and vertical functions to build complete corridors between rooms
    fn build_corridors(&mut self, rng: &mut brac::RandomNumberGenerator) {
        let mut rooms = self.rooms.clone(); // clone to modify the new collection without changing the original
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x)); // sort the rooms by the x coordinates of their centers

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.build_horizontal_tunnel(prev.x, new.x, prev.y);
                self.build_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.build_vertical_tunnel(prev.y, new.y, new.x);
                self.build_horizontal_tunnel(prev.x, new.x, prev.y);
            }
        }
    }
    pub fn new(rng: &mut brac::RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: map::Map::new(),
            rooms: Vec::new(),
            player_start: brac::Point::zero(),
        };

        map_builder.fill(map::TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start = map_builder.rooms[0].center();  // set the player start to the center of the first room
        map_builder
    }
}






















































