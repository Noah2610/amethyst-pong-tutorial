mod bounce;
mod decrease_velocities;
mod limit_velocities;
mod move_entities;
mod paddle_control;
// mod rotator;
mod scale_sprites;
mod scoring;

pub mod prelude {
    pub use super::BounceSystem;
    pub use super::DecreaseVelocitiesSystem;
    pub use super::LimitVelocitiesSystem;
    pub use super::MoveEntitiesSystem;
    pub use super::PaddleControlSystem;
    // pub use super::RotatorSystem;
    pub use super::ScaleSpritesSystem;
    pub use super::ScoringSystem;
}

pub use bounce::BounceSystem;
pub use decrease_velocities::DecreaseVelocitiesSystem;
pub use limit_velocities::LimitVelocitiesSystem;
pub use move_entities::MoveEntitiesSystem;
pub use paddle_control::PaddleControlSystem;
// pub use rotator::RotatorSystem;
pub use scale_sprites::ScaleSpritesSystem;
pub use scoring::ScoringSystem;
