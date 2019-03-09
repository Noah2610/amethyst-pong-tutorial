use amethyst::ecs::{Component, DenseVecStorage};

pub struct MaxVelocity {
    pub x: f32,
    pub y: f32,
}

impl MaxVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for MaxVelocity {
    type Storage = DenseVecStorage<Self>;
}
