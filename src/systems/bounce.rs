use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;
use crate::pong::constants::*;
use crate::rect::Rect;

pub struct BounceSystem;

impl<'a> System<'a> for BounceSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
    );

    fn run(
        &mut self,
        (balls, paddles, mut velocities, transforms, sizes): Self::SystemData,
    ) {
        // Check whether a ball collided, and bounce off accordingly

        for (ball, ball_velocity, ball_transform) in
            (&balls, &mut velocities, &transforms).join()
        {
            let ball_x = ball_transform.translation().x;
            let ball_y = ball_transform.translation().y;

            // Bounce at the top or bottom arena
            if (ball_y >= ARENA_HEIGHT - ball.radius && ball_velocity.y > 0.0)
                || (ball_y <= ball.radius && ball_velocity.y < 0.0)
            {
                ball_velocity.y = -ball_velocity.y;
            }

            // Bounce at the paddles
            for (paddle, paddle_transform, paddle_size) in
                (&paddles, &transforms, &sizes).join()
            {
                let paddle_x =
                    paddle_transform.translation().x - paddle_size.w * 0.5;
                let paddle_y =
                    paddle_transform.translation().y - paddle_size.h * 0.5;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its centre is within the larger wrapper
                // rectangle.

                // Something with the collision seems off, kinda offset
                if point_in_rect(
                    ball_transform,
                    Rect {
                        top:    paddle_y + paddle_size.h + ball.radius,
                        bottom: paddle_y - ball.radius,
                        left:   paddle_x - ball.radius,
                        right:  paddle_x + paddle_size.w + ball.radius,
                    },
                ) {
                    if (paddle.side == Side::Left && ball_velocity.x < 0.0)
                        || (paddle.side == Side::Right && ball_velocity.x > 0.0)
                    {
                        ball_velocity.x = -ball_velocity.x;
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(transform: &Transform, rect: Rect) -> bool {
    let pos = transform.translation();
    pos.x >= rect.left
        && pos.x <= rect.right
        && pos.y >= rect.bottom
        && pos.y <= rect.top
}
