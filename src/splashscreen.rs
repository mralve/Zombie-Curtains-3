use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    utils::removal::Removal,
    window::ScreenDimensions,
};

use crate::miscfunc;
use crate::zombie_curtains::ZombieCurtains;

pub struct SplashScreen;

impl SimpleState for SplashScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Removal<i32>>();
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        miscfunc::init_camera(world, &dimensions, 1.0);

        let sprites = miscfunc::load_spritesheet(world, "sprites/logo");

        miscfunc::init_sprite(world, sprites.clone(), 0, 0., 0., 0.);

        miscfunc::init_sprite(world, sprites.clone(), 1, 0., 0. - 40.0, 0.);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Press enter = start the game by switching state to in-game state
            if is_key_down(&event, VirtualKeyCode::Return) {
                return Trans::Switch(Box::new(ZombieCurtains));
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}
