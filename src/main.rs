extern crate amethyst;

mod pong;

mod components;
mod systems;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::utils::application_root_dir;

use pong::Pong;
use systems::prelude::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let render_bundle = RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor();
    let transform_bundle = TransformBundle::new();
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(
            PaddleControlSystem,
            "paddle_control_system",
            &["input_system"],
        )
        .with(
            LimitVelocitiesSystem,
            "limit_velocities_system",
            &["paddle_control_system"],
        )
        .with(
            MoveEntitiesSystem,
            "move_entities_system",
            &["limit_velocities_system"],
        )
        .with(
            DecreaseVelocitiesSystem,
            "decrease_velocites_system",
            &["move_entities_system"],
        );
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    Ok(())
}
