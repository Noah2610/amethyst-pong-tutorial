use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub mod prelude {
    pub use super::ScoreBoard;
    pub use super::ScoreText;
}

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub p1_score: u32,
    pub p2_score: u32,
}

/// ScoreText contains the UI text components that display the score
pub struct ScoreText {
    pub p1_text: Entity,
    pub p2_text: Entity,
}

impl ScoreText {
    pub fn new(p1_text: Entity, p2_text: Entity) -> Self {
        Self { p1_text, p2_text }
    }
}
