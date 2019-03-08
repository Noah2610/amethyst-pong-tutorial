mod decrease_velocities;
mod move_entities;
mod paddle_control;

pub mod prelude {
    pub use super::DecreaseVelocitiesSystem;
    pub use super::MoveEntitiesSystem;
    pub use super::PaddleControlSystem;
}

pub use decrease_velocities::DecreaseVelocitiesSystem;
pub use move_entities::MoveEntitiesSystem;
pub use paddle_control::PaddleControlSystem;
