mod helpers;

use amethyst::prelude::*;

use helpers::*;

pub mod constants {
    pub const ARENA_WIDTH: f32 = 100.0;
    pub const ARENA_HEIGHT: f32 = 100.0;

    pub const PADDLE_WIDTH: f32 = 4.0;
    pub const PADDLE_HEIGHT: f32 = 16.0;

    pub const BALL_VELOCITY: [f32; 2] = [75.0, 50.0];
    pub const BALL_RADIUS: f32 = 2.0;
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let spritesheet_handle = load_spritesheet(world);

        initialize_ball(world, spritesheet_handle.clone());
        initialize_paddles(world, spritesheet_handle);
        initialize_camera(world);
    }
}
