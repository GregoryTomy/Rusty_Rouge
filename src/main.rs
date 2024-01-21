#![warn(clippy::all, clippy::pedantic)]

pub mod map;
pub mod player;
pub mod map_builder;
mod camera;

use bracket_lib::prelude as brac;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;    // the smaller viewport into the world
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = brac::RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

impl brac::GameState for State {
    fn tick(&mut self, context: &mut brac::BTerm) {
        context.set_active_console(0);
        context.cls();
        context.set_active_console(1);
        context.cls();
        self.player.update(context, &self.map, &mut self.camera);
        self.map.render(context, &self.camera);
        self.player.render(context, &self.camera);
    }
}


fn main() -> brac::BError {
    // code creates a terminal with two console layers, one for the map, one for the player
    let context = brac::BTermBuilder::new()         //generic terminal
        .with_title("Rusty Rouge")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // specify the size of the subsequent consoles
        .with_tile_dimensions(32, 32)   // size of the character in the font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")      // add console
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // add second console with no background so transparency shoes through it
        .build()?;

    brac::main_loop(context, State::new())
}


















