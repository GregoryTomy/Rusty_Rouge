use bracket_lib::prelude as brac;
use crate::map;

pub struct Player {
    pub position: brac::Point,
}

impl Player {
    pub fn new(position: brac::Point) -> Self {
        Self {
            position,
        }
    }

    pub fn render(&self, context: &mut brac::BTerm) {
        context.set(
            self.position.x, self.position.y, brac::WHITE, brac::BLACK, brac::to_cp437('@')
        );
    }

    // implementation of player movement
    pub fn update(&mut self, context: &mut brac::BTerm, map: &map::Map) {
        if let Some(key) = context.key {
            // delta stores the intended change in player position
            let delta = match key {
                brac::VirtualKeyCode::A => brac::Point::new(-1, 0),
                brac::VirtualKeyCode::D => brac::Point::new(1, 0),
                brac::VirtualKeyCode::W => brac::Point::new(0, -1),
                brac::VirtualKeyCode::S => brac::Point::new(0, 1),
                _ => brac::Point::zero(), // no move requested
            };

            // calculate player's new position
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }

        }
    }
}



















































