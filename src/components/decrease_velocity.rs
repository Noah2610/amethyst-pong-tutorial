use amethyst::ecs::{Component, DenseVecStorage};

pub struct DecreaseVelocity {
    pub x: f32,
    pub y: f32,
}

impl DecreaseVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for DecreaseVelocity {
    type Storage = DenseVecStorage<Self>;
}
