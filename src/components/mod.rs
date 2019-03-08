mod ball;
mod paddle;

pub mod prelude {
    pub use super::Ball;
    pub use super::Paddle;
    pub use super::Side;
}

pub use ball::Ball;
pub use paddle::{Paddle, Side};
