use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
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

use crate::components::{Paddle, Side};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Paddle>();

        let spritesheet_handle = load_spritesheet(world);

        initialize_paddles(world, spritesheet_handle);
        initialize_camera(world);
    }
}

/// Initialize a camera entity
fn initialize_camera(world: &mut World) {
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
fn initialize_paddles(world: &mut World, spritesheet: SpriteSheetHandle) {
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
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create the right paddle entity
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .build();
}

/// Load the spritesheet for the game
fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
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
