use amethyst::ecs::{Component, DenseVecStorage};

pub struct Scale(pub bool);

impl Scale {
    pub fn new() -> Self {
        Self(false)
    }
}

impl Component for Scale {
    type Storage = DenseVecStorage<Self>;
}
