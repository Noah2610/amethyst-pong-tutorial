use amethyst::core::timing::Time;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;

/// Decrease entities' velocities
pub struct DecreaseVelocitiesSystem;

impl<'s> System<'s> for DecreaseVelocitiesSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, DecreaseVelocity>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (time, decreases, paddles, mut velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        // NON-PADDLES
        for (decrease, velocity, _) in
            (&decreases, &mut velocities, !&paddles).join()
        {
            decrease_velocity_for(dt, velocity, decrease);
        }

        // PADDLES
        for (decrease, velocity, paddle) in
            (&decreases, &mut velocities, &paddles).join()
        {
            if !paddle.has_moved {
                decrease_velocity_for(dt, velocity, decrease);
            }
        }
    }
}

fn decrease_velocity_for(
    dt: f32,
    velocity: &mut Velocity,
    decrease: &DecreaseVelocity,
) {
    // TODO: Not sure if deltatime is needed here
    // let decr_x = decrease.x * dt;
    // let decr_y = decrease.y * dt;
    let decr_x = decrease.x;
    let decr_y = decrease.y;
    let sign_x = velocity.x.signum();
    let sign_y = velocity.y.signum();

    if velocity.x > 0.0 {
        velocity.x -= decr_x;
    } else if velocity.x < 0.0 {
        velocity.x += decr_x;
    }
    if velocity.x.signum() != sign_x {
        velocity.x = 0.0;
    }

    if velocity.y > 0.0 {
        velocity.y -= decr_y;
    } else if velocity.y < 0.0 {
        velocity.y += decr_y;
    }
    if velocity.y.signum() != sign_y {
        velocity.y = 0.0;
    }
}
