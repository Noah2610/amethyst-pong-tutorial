use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, ReadExpect, System, Write, WriteStorage};
use amethyst::ui::UiText;

use crate::components::prelude::*;
use crate::pong::constants::*;
use crate::pong::scoreboard::prelude::*;

const MAX_SCORE: u32 = 999;

pub struct ScoringSystem;

impl<'a> System<'a> for ScoringSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, UiText>,
        Write<'a, ScoreBoard>,
        ReadExpect<'a, ScoreText>,
    );

    fn run(
        &mut self,
        (
            mut balls,
            mut velocities,
            mut transforms,
            mut ui_texts,
            mut scoreboard,
            scoretext,
        ): Self::SystemData,
    ) {
        for (ball, velocity, transform) in
            (&mut balls, &mut velocities, &mut transforms).join()
        {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Left side goal - right player scores
                scoreboard.p2_score = (scoreboard.p2_score + 1).min(MAX_SCORE);
                // Here we get the UiText component (?) using our ScoreText resource.
                if let Some(text) = ui_texts.get_mut(scoretext.p2_text) {
                    text.text = scoreboard.p2_score.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Right side goal - left player scores
                scoreboard.p1_score = (scoreboard.p1_score + 1).min(MAX_SCORE);
                // Here we get the UiText component (?) using our ScoreText resource.
                if let Some(text) = ui_texts.get_mut(scoretext.p1_text) {
                    text.text = scoreboard.p1_score.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                velocity.x = -velocity.x; // Reverse direction
                transform.set_x(ARENA_WIDTH * 0.5); // Reset position

                print_score(&scoreboard);
            }
        }
    }
}

fn print_score(scoreboard: &ScoreBoard) {
    println!(
        r#"Score:
  player one: {:^3}
  player two: {:^3}"#,
        scoreboard.p1_score, scoreboard.p2_score,
    );
}
