use std::time::Instant;

use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, System, WriteStorage};

use crate::components::prelude::*;
use crate::pong::constants::*;
use crate::pong::scoreboard::prelude::*;

pub struct RotatorSystem;

impl<'a> System<'a> for RotatorSystem {
    type SystemData = (WriteStorage<'a, Rotate>, WriteStorage<'a, Transform>);

    fn run(&mut self, (mut rotatables, mut transforms): Self::SystemData) {
        let now = Instant::now();
        for (rotatable, transform) in (&mut rotatables, &mut transforms).join()
        {
            if now - rotatable.last_rotation > rotatable.delay {
                rotatable.rotation += rotatable.amount;
                transform.set_rotation(rotatable.get_rotation());
                rotatable.last_rotation = now;
                dbg!(rotatable.rotation);
            }
        }
    }
}
