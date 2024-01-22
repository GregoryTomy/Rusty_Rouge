mod player_input;
mod map_renders;
mod entity_render;

use crate::prelude::*;

// function creates a Legion Schedule - an execution plan for the systems.
pub fn build_scheduler() -> Schedule {
    Schedule::builder()             // starts the systems building process
        .add_system(player_input::player_input_system())
        .add_system(map_renders::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()                    // ends the process
}