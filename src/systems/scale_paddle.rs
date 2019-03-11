use amethyst::assets::AssetStorage;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{
    Join,
    Read,
    ReadExpect,
    ReadStorage,
    System,
    WriteStorage,
};
use amethyst::renderer::{SpriteRender, SpriteSheet, SpriteSheetHandle};

use crate::components::prelude::*;
use crate::pong::constants::*;

pub struct ScalePaddleSystem {
    scaled_once: bool,
}

impl ScalePaddleSystem {
    pub fn new() -> Self {
        Self { scaled_once: false }
    }
}

impl<'s> System<'s> for ScalePaddleSystem {
    type SystemData = (
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, SpriteSheetHandle>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (spritesheet, spritesheet_handle, paddles, sprites, mut transforms): Self::SystemData,
    ) {
        if self.scaled_once {
            return;
        }
        if let Some(sprite_sheet) = spritesheet.get(&spritesheet_handle) {
            let size = (
                sprite_sheet.sprites.get(0).expect("unwrap sprite").width,
                sprite_sheet.sprites.get(0).expect("unwrap sprite").height,
            );

            let scale = (PADDLE_WIDTH / size.0, PADDLE_HEIGHT / size.1);

            self.scaled_once = true;
            for (_, paddle, transform) in
                (&sprites, &paddles, &mut transforms).join()
            {
                transform.set_scale(scale.0, scale.1, 0.0);
            }
        }
    }
}
