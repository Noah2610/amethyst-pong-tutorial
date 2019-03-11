use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl From<[f32; 2]> for Velocity {
    fn from(vel: [f32; 2]) -> Self {
        Self {
            x: vel[0],
            y: vel[1],
        }
    }
}
