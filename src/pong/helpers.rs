use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera,
    Flipped,
    PngFormat,
    Projection,
    SpriteRender,
    SpriteSheet,
    SpriteSheetFormat,
    SpriteSheetHandle,
    Texture,
    TextureMetadata,
};
use amethyst::utils::application_root_dir;

use super::constants::*;
use crate::components::prelude::*;

/// Load the spritesheet for the game
pub fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}/texture/pong_spritesheet.png", application_root_dir()),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}/texture/pong_spritesheet.ron", application_root_dir()),
        SpriteSheetFormat,
        texture_handle,
        (),
        &spritesheet_store,
    )
}

/// Initialize a camera entity
pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,          // Left
            ARENA_WIDTH,  // Right
            0.0,          // Bottom (!)
            ARENA_HEIGHT, // Top    (!)
        )))
        .with(transform)
        .build();
}

/// Initialize one paddle on the left, and one paddle on the right
pub fn initialize_paddles(world: &mut World, spritesheet: SpriteSheetHandle) {
    // Create translations
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Paddle sprites
    let sprite_render = SpriteRender {
        sprite_sheet:  spritesheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the spritesheet
    };

    // Create the left paddle entity
    world
        .create_entity()
        .with(Paddle::new(Side::Left, PADDLE_SPEED))
        .with(Size::new(PADDLE_WIDTH, PADDLE_HEIGHT))
        .with(Velocity::empty())
        .with(DecreaseVelocity::new(0.0, PADDLE_VELOCITY_DECREASE))
        .with(MaxVelocity::new(
            PADDLE_MAX_VELOCITY_X,
            PADDLE_MAX_VELOCITY_Y,
        ))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create the right paddle entity
    world
        .create_entity()
        .with(Paddle::new(Side::Right, PADDLE_SPEED))
        .with(Size::new(PADDLE_WIDTH, PADDLE_HEIGHT))
        .with(Velocity::empty())
        .with(DecreaseVelocity::new(0.0, PADDLE_VELOCITY_DECREASE))
        .with(MaxVelocity::new(
            PADDLE_MAX_VELOCITY_X,
            PADDLE_MAX_VELOCITY_Y,
        ))
        .with(right_transform)
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .build();
}

/// Initialize a ball entity
pub fn initialize_ball(world: &mut World, spritesheet: SpriteSheetHandle) {
    // Create translation
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Ball sprite
    let sprite_render = SpriteRender {
        sprite_sheet:  spritesheet,
        sprite_number: 1, // ball is the second sprite in the spritesheet
    };

    // Create entity
    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::with_radius(BALL_RADIUS))
        .with(Velocity::from(BALL_VELOCITY))
        .with(local_transform)
        .build();
}
