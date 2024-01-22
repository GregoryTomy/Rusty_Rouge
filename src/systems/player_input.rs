use crate::prelude::*;

#[system]                   // annotates the player_input function with a procedural macro named system
#[write_component(Point)]   // requests writable access to component type
#[read_component(Player)]   // requests read-only access to component type
pub fn player_input(
    ecs: &mut SubWorld,     // like World but can only see the components requested
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
)
{
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()
                .filter(component::<Player>()); // query the entity with Point component (returned) and Player component (not returned)

            // run the query defined above and place results in an iterator
            players.iter_mut(ecs).for_each(
                |pos| {
                    let destination = *pos + delta;
                    if map.can_enter_tile(destination) {
                        *pos = destination;
                        camera.on_player_move(destination);
                    }
                }
            )
        }
    }
}