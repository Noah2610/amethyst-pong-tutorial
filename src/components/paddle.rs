use std::fmt::{self, Display};

use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side:      Side,
    pub speed:     f32,
    pub has_moved: bool,
}

impl Paddle {
    pub fn new(side: Side, speed: f32) -> Self {
        Paddle {
            side,
            speed,
            has_moved: false,
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
