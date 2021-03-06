use amethyst::ecs::{Component, DenseVecStorage};

pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

impl Component for Size {
    type Storage = DenseVecStorage<Self>;
}
