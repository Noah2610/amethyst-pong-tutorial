use std::fmt::{self, Display};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Paddle { side }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl Display for Side {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Side::*;
        write!(
            fmt,
            "{}",
            match self {
                Left => "Left Paddle",
                Right => "Right Paddle",
            }
        )
    }
}
