// A system that renders all entities with both Point and Render component

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()    // query all entities that have Point and Render component
        .iter(ecs)
        .for_each(
            |(position, render)| {
                draw_batch.set(
                    *position - offset,
                    render.color,
                    render.glyph,
                );
            }
        );
    // 5000 is used as a sort oder because the map may include 4000 elements. Leaving room in case the map changes or we add other user interface elements
    draw_batch.submit(5000).expect("Batch error");
}
