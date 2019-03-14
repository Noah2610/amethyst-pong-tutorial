use amethyst::State;
use amethyst::StateEvent;

use crate::custom_game_data::prelude::*;

pub struct Running;
pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
}
