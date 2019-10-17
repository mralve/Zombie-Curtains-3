use amethyst::{
    prelude::*,
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    window::ScreenDimensions,
    utils::removal::exec_removal,
};

use crate::miscfunc;

pub struct ZombieCurtains;

impl SimpleState for ZombieCurtains {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Remove all entities with the RemovalId value of Something.
        exec_removal(&world.entities(), &world.read_storage(), -1);

        // Force the world to be up to date. This is normally called automatically at the end of the
        // frame by amethyst.
        world.maintain();
        
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        miscfunc::init_camera(world, &dimensions);

        println!("In-game state now active!");

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

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                //info!("handling key event: {:?}", event);
            }

        }

        // Keep going
        Trans::None
    }
}
