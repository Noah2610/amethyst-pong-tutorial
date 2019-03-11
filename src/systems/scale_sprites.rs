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

pub struct ScaleSpritesSystem;

impl<'s> System<'s> for ScaleSpritesSystem {
    type SystemData = (
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadStorage<'s, Size>,
        ReadStorage<'s, SpriteRender>,
        WriteStorage<'s, Scale>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (spritesheet, sizes, sprites, mut scales, mut transforms): Self::SystemData,
    ) {
        for (size, scale_component, transform, sprite_render) in
            (&sizes, &mut scales, &mut transforms, &sprites).join()
        {
            if scale_component.0 {
                continue;
            }
            let spritesheet_handle = &sprite_render.sprite_sheet;
            let sprite_id = sprite_render.sprite_number;
            if let Some(spritesheet) = spritesheet.get(spritesheet_handle) {
                let sprite =
                    spritesheet.sprites.get(sprite_id).expect(&format!(
                        "Couldn't get sprite #{} from spritesheet #{}",
                        sprite_id,
                        spritesheet_handle.id()
                    ));
                let sprite_w = sprite.width;
                let sprite_h = sprite.height;
                let scale = [size.w / sprite_w, size.h / sprite_h];
                transform.set_scale(scale[0], scale[1], 0.0);
                scale_component.0 = true;
            }
        }
    }
}
