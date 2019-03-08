use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ball {
    pub radius: f32,
}

impl Ball {
    pub fn with_radius(radius: f32) -> Self {
        Self { radius }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
