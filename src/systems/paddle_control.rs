use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{Paddle, Side, Velocity};
use crate::pong::constants::*;

pub struct PaddleControlSystem;

impl<'s> System<'s> for PaddleControlSystem {
    type SystemData = (
        // Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (input, paddles, mut velocities): Self::SystemData) {
        for (paddle, velocity) in (&paddles, &mut velocities).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = mv_amount as f32 * paddle.speed;
                    velocity.y += scaled_amount;
                }
            }
        }
    }
}
