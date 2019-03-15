extern crate amethyst;

mod resource_helpers;

mod custom_game_data;
mod pong;
mod rect;

mod components;
mod systems;

pub use self::resource_helpers::*;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::{LogLevelFilter, LoggerConfig};

use custom_game_data::prelude::*;
use pong::Game;
use systems::prelude::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });

    let display_path = resource("config/display_config.ron");
    let binding_path = resource("config/bindings_config.ron");

    let config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let render_bundle =
        RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor();
    let transform_bundle = TransformBundle::new();
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    // let game_data = GameDataBuilder::default()
    let game_data = CustomGameDataBuilder::default()
        .with_base_bundle(render_bundle)?
        .with_base_bundle(transform_bundle)?
        .with_base_bundle(input_bundle)?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_running(
            PaddleControlSystem,
            "paddle_control_system",
            //&["input_system"],  // TODO: Figure out how to get this dependency check to work
                                  //       again, using CustomGameDataBuilder
            &[],
        )
        .with_running(
            LimitVelocitiesSystem,
            "limit_velocities_system",
            &["paddle_control_system"],
        )
        .with_running(
            MoveEntitiesSystem,
            "move_entities_system",
            &["limit_velocities_system"],
        )
        .with_running(
            BounceSystem,
            "ball_bounce_system",
            &["move_entities_system"],
        )
        .with_running(
            DecreaseVelocitiesSystem,
            "decrease_velocites_system",
            &["move_entities_system"],
        )
        .with_running(ScoringSystem, "scoring_system", &["move_entities_system"])
        .with_running(ScaleSpritesSystem, "scale_sprites_system", &[])
        // .with(RotatorSystem, "rotate_system", &[])
        ;
    let mut game = Application::new("./", Game, game_data)?;
    game.run();

    Ok(())
}
