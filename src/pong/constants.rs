pub mod keys {
    use amethyst::renderer::VirtualKeyCode;

    pub const QUIT_KEY: VirtualKeyCode = VirtualKeyCode::Escape;
    pub const PAUSE_KEY: VirtualKeyCode = VirtualKeyCode::P;
}

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 32.0;
pub const PADDLE_SPEED: f32 = 2.0;
pub const PADDLE_VELOCITY_DECREASE: f32 = 2.0;
pub const PADDLE_MAX_VELOCITY_X: f32 = 0.0;
pub const PADDLE_MAX_VELOCITY_Y: f32 = 2.0;

pub const BALL_VELOCITY: [f32; 2] = [30.0, 30.0];
pub const BALL_RADIUS: f32 = 2.0;
pub const BALL_SPEED_INCR: [f32; 2] = [5.0, 5.0];
// pub const BALL_ROTATE_AMOUNT: f32 = 10.0;
// pub const BALL_ROTATE_DELAY_MS: u64 = 50;
