pub mod scoreboard;

mod helpers;

use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{SpriteSheetHandle, VirtualKeyCode};
use amethyst::{State, StateData, StateEvent, Trans};

use crate::custom_game_data::prelude::*;

use helpers::*;

pub mod constants;

pub struct Game;
pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Game {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;

        use crate::components::prelude::*;
        world.register::<Ball>();

        let spritesheet_handle = load_spritesheet(world);

        initialize_ball(world, spritesheet_handle.clone());
        initialize_paddles(world, spritesheet_handle.clone());
        initialize_camera(world);

        initialize_scoreboard(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        use constants::keys::*;

        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, QUIT_KEY) {
                Trans::Quit
            } else if is_key_down(&event, PAUSE_KEY) {
                Trans::Push(Box::new(Paused))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        // Create paused UI
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        use constants::keys::*;

        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, QUIT_KEY) {
                Trans::Quit
            } else if is_key_down(&event, PAUSE_KEY) {
                // Remove paused UI
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false); // `false` to say that the main game is not running (paused)
        Trans::None
    }
}
