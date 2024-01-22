#![warn(clippy::all, clippy::pedantic)]

pub mod map;
pub mod map_builder;
mod camera;
mod components;
mod spawning_entities;
mod systems;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    // the smaller viewport into the world
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawning_entities::*;
    pub use crate::systems::*;
    pub use legion::*;
    pub use bracket_lib::prelude::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
}

use prelude::*;

struct State {
    ecs: World,
    // stores all entities and components
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);

        // spawn one monster per room except in the first room with the player
        map_builder.rooms
            .iter()
            .skip(1)    // skip the first room
            .map(|r| r.center())    // transform each entry from room to the result of center(), a Point using map()
            .for_each(|position| spawn_enemy(&mut ecs, &mut rng, position));


        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        self.resources.insert(context.key);  // add the keyboard state as a resource
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(context).expect("Render error");
    }
}


fn main() -> BError {
    // code creates a terminal with two console layers, one for the map, one for the player
    let context = BTermBuilder::new()         //generic terminal
        .with_title("Rusty Rouge")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // specify the size of the subsequent consoles
        .with_tile_dimensions(32, 32)   // size of the character in the font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")      // add console
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // add second console with no background so transparency shoes through it
        .build()?;

    main_loop(context, State::new())
}


















