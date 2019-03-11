use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;

use crate::pong::constants::*;

pub struct ScoringSystem;

impl<'a> System<'a> for ScoringSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (mut balls, mut velocities, mut transforms): Self::SystemData,
    ) {
        for (ball, velocity, transform) in
            (&mut balls, &mut velocities, &mut transforms).join()
        {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Left side goal - right player scores
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Right side goal - left player scores
                true
            } else {
                false
            };

            if did_hit {
                velocity.x = -velocity.x; // Reverse direction
                transform.set_x(ARENA_WIDTH * 0.5); // Reset position
            }
        }
    }
}
