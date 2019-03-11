mod helpers;
mod scoreboard;

use amethyst::prelude::*;

use helpers::*;

pub mod constants {
    pub const ARENA_WIDTH: f32 = 100.0;
    pub const ARENA_HEIGHT: f32 = 100.0;

    pub const PADDLE_WIDTH: f32 = 4.0;
    pub const PADDLE_HEIGHT: f32 = 16.0;
    pub const PADDLE_SPEED: f32 = 2.0;
    pub const PADDLE_VELOCITY_DECREASE: f32 = 2.0;
    pub const PADDLE_MAX_VELOCITY_X: f32 = 0.0;
    pub const PADDLE_MAX_VELOCITY_Y: f32 = 2.0;

    pub const BALL_VELOCITY: [f32; 2] = [30.0, 50.0];
    pub const BALL_RADIUS: f32 = 2.0;
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        use crate::components::prelude::*;
        world.register::<Ball>();

        let spritesheet_handle = load_spritesheet(world);

        initialize_ball(world, spritesheet_handle.clone());
        initialize_paddles(world, spritesheet_handle);
        initialize_camera(world);

        initialize_scoreboard(world);
    }
}
