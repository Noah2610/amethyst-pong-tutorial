pub mod scoreboard;

mod helpers;

use amethyst::prelude::*;
use amethyst::renderer::SpriteSheetHandle;

use helpers::*;

pub mod constants {
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
    pub const BALL_ROTATE_AMOUNT: f32 = 10.0;
    pub const BALL_ROTATE_DELAY_MS: u64 = 50;
}

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        use crate::components::prelude::*;
        world.register::<Ball>();

        let spritesheet_handle = load_spritesheet(world);

        initialize_ball(world, spritesheet_handle.clone());
        initialize_paddles(world, spritesheet_handle.clone());
        initialize_camera(world);

        initialize_scoreboard(world);

        world.register::<SpriteSheetHandle>();
        world.add_resource(spritesheet_handle);
    }
}
