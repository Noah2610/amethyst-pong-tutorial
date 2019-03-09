use amethyst::core::timing::Time;
use amethyst::ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::prelude::*;

pub struct LimitVelocitiesSystem;

impl<'s> System<'s> for LimitVelocitiesSystem {
    type SystemData = (ReadStorage<'s, MaxVelocity>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (max_velocities, mut velocities): Self::SystemData) {
        for (max_vel, vel) in (&max_velocities, &mut velocities).join() {
            if vel.x > 0.0 {
                vel.x = vel.x.min(max_vel.x);
            } else if vel.x < 0.0 {
                vel.x = (-vel.x).min(-max_vel.x);
            }
            if vel.y > 0.0 {
                vel.y = vel.y.min(max_vel.y);
            } else if vel.y < 0.0 {
                vel.y = vel.y.max(-max_vel.y);
            }
        }
    }
}
