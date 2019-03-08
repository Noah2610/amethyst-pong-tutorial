use std::fmt::{self, Display};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side:   Side,
    pub width:  f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
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
