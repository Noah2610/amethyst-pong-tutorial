use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{Paddle, Side};
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH, PADDLE_HEIGHT};

pub const PADDLE_SPEED: f64 = 70.0;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
    );

    fn run(&mut self, (time, input, mut transforms, paddles): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let paddle_y = transform.translation().y;
                    // let scaled_amount =
                    //     (time.delta_time().as_float_secs() * (mv_amount * PADDLE_SPEED)) as f32;
                    let scaled_amount =
                        (time.delta_time().as_millis() as f64 * 0.001 * (mv_amount * PADDLE_SPEED))
                            as f32;
                    transform.set_y(
                        (paddle_y + scaled_amount)
                            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                            .max(PADDLE_HEIGHT * 0.5),
                    );
                }
            }
        }
    }
}
