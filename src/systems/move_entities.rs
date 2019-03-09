use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;
use crate::pong::constants::*;

/// Move all entites with `Transform` and `Velocity`
pub struct MoveEntitiesSystem;

impl<'s> System<'s> for MoveEntitiesSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Size>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, velocities, sizes, mut transforms): Self::SystemData) {
        // Move every entity, according to its speed, and the time passed
        let dt = time.delta_seconds();
        // WITHOUT a Size
        for (velocity, transform, _) in (&velocities, &mut transforms, !&sizes).join() {
            transform.translate_x(velocity.x * dt);
            transform.translate_y(velocity.y * dt);
        }

        // WITH a Size - clip to arena
        for (velocity, transform, size) in (&velocities, &mut transforms, &sizes).join() {
            let pos_x = transform.translation().x;
            let pos_y = transform.translation().y;
            transform.set_x(
                (pos_x + velocity.x)
                    .min(ARENA_WIDTH - size.w * 0.5)
                    .max(size.w * 0.5),
            );
            transform.set_y(
                (pos_y + velocity.y)
                    .min(ARENA_HEIGHT - size.h * 0.5)
                    .max(size.h * 0.5),
            );
        }
    }
}
