#![warn(clippy::all, clippy::pedantic)]

pub mod map;

mod prelude {
    pub use bracket_lib::prelude as brac;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map;
}

use prelude::*;

struct State {
    map: map::Map,
}

impl State {
    fn new() -> Self {
        Self { map: map::Map::new() }
    }
}

impl brac::GameState for State {
    fn tick(&mut self, context: &mut brac::BTerm) {
        context.cls();
        self.map.render(context);
    }
}


fn main() -> brac::BError {
    let context = brac::BTermBuilder::simple80x50()
        .with_title("Rusty Rouge")
        .with_fps_cap(30.0)
        .build()?;

    brac::main_loop(context, State::new())
}
