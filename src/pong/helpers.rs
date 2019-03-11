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
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use super::constants::*;
use super::scoreboard::prelude::*;
use crate::components::prelude::*;
use crate::resource;

/// Load the spritesheet for the game
pub fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            resource("textures/pong_spritesheet.png"),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        resource("textures/pong_spritesheet.ron"),
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
    use amethyst::assets::Handle;
    use amethyst::core::nalgebra::{Translation3, UnitQuaternion, Vector3};

    // let coords = {
    //     // Immutable world borrow starts here
    //     let sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    //     let sprite_sheet = sheet_store.get(&spritesheet);
    //     (
    //         sprite_sheet
    //             .expect("unwrap sprite_sheet")
    //             .sprites
    //             .get(1)
    //             .expect("unwrap sprite")
    //             .width,
    //         sprite_sheet
    //             .expect("unwrap sprite_sheet")
    //             .sprites
    //             .get(1)
    //             .expect("unwrap sprite")
    //             .height,
    //     )
    // }; // Immutable world borrow ends here
    // coords.0 is x, coords.1 is y

    // dbg!(coords);

    // Create translations
    // let mut left_transform = Transform::new(
    //     Translation3::new(0.0, 0.0, 0.0),
    //     UnitQuaternion::new(Vector3::new(0.0, 0.0, 0.0)), // rotation
    //     Vector3::new(1.0, 1.0, 0.0),                      // scale
    // );
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles
    let y = ARENA_HEIGHT * 0.5;
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
        // .with(Rotate::new(BALL_ROTATE_AMOUNT, BALL_ROTATE_DELAY_MS))
        .with(local_transform)
        .build();
}

/// Initialize the scoreboard
pub fn initialize_scoreboard(world: &mut World) {
    const FONT_SIZE: f32 = 50.0;

    let font = world.read_resource::<Loader>().load(
        resource("fonts/square.ttf"),
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let p1_transform = new_ui_transform(
        "P1",
        Anchor::TopMiddle,
        (-50.0, -50.0, 1.0, 200.0, 50.0, 0),
    );
    let p2_transform = new_ui_transform(
        "P2",
        Anchor::TopMiddle,
        (50.0, -50.0, 1.0, 200.0, 50.0, 0),
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            FONT_SIZE,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            FONT_SIZE,
        ))
        .build();

    world.add_resource(ScoreText::new(p1_score, p2_score));
}

/// `UiTransform::new` wrapper
fn new_ui_transform<T: ToString>(
    name: T,
    anchor: Anchor,
    pos: (f32, f32, f32, f32, f32, i32),
) -> UiTransform {
    UiTransform::new(
        name.to_string(),
        anchor,
        pos.0, // x
        pos.1, // y
        pos.2, // z
        pos.3, // width
        pos.4, // height
        pos.5, // tab-order (?)
    )
}
