#![warn(clippy::all, clippy::pedantic)]

pub mod map;
pub mod player;
pub mod map_builder;

use bracket_lib::prelude as brac;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map;
    pub use crate::player;
    pub use crate::map_builder;
}

use prelude::*;

struct State {
    map: map::Map,
    player: player::Player,
}

impl State {
    fn new() -> Self {
        let mut rng = brac::RandomNumberGenerator::new();
        let map_builder = map_builder::MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: player::Player::new(map_builder.player_start),
        }
    }
}

impl brac::GameState for State {
    fn tick(&mut self, context: &mut brac::BTerm) {
        context.cls();
        self.player.update(context, &self.map);
        self.map.render(context);
        self.player.render(context);
    }
}


fn main() -> brac::BError {
    let context = brac::BTermBuilder::simple80x50()
        .with_title("Rusty Rouge")
        .with_fps_cap(30.0)
        .build()?;

    brac::main_loop(context, State::new())
}


















