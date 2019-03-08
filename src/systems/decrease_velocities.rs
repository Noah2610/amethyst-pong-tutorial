use amethyst::core::timing::Time;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;

/// Decrease entities' velocities
pub struct DecreaseVelocitiesSystem;

impl<'s> System<'s> for DecreaseVelocitiesSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, DecreaseVelocity>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (time, decreases, mut velocities): Self::SystemData) {
        let dt = time.delta_seconds();

        for (decrease, velocity) in (&decreases, &mut velocities).join() {
            let decr_x = decrease.x * dt;
            let decr_y = decrease.y * dt;
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
                println!("HERE");
                velocity.y = 0.0;
            }
        }
    }
}
