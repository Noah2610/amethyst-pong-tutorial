use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::Ball;

pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, balls, mut transforms): Self::SystemData) {
        // Move every ball, according to its speed, and the time passed
        let dt = time.delta_seconds();
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.translate_x(ball.velocity[0] * dt);
            transform.translate_y(ball.velocity[1] * dt);
        }
    }
}
