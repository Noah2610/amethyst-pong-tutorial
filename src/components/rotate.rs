use std::time::{Duration, Instant};

use amethyst::core::nalgebra::{UnitQuaternion, Vector3};
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Rotate {
    pub rotation:      f32,
    pub amount:        f32,
    pub delay:         Duration,
    pub last_rotation: Instant,
}

impl Rotate {
    pub fn new(amount: f32, duration_ms: u64) -> Self {
        Self {
            // amount:        UnitQuaternion::new(Vector3::new(
            //     amount, amount, amount,
            // )),
            rotation: 0.0,
            amount,
            delay: Duration::from_millis(duration_ms),
            last_rotation: Instant::now(),
        }
    }

    pub fn get_rotation(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::new(Vector3::new(self.amount, 0.0, 0.0))
    }
}

impl Component for Rotate {
    type Storage = DenseVecStorage<Self>;
}
