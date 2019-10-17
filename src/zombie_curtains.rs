use amethyst::{
    prelude::*,
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    window::ScreenDimensions,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    utils::removal::exec_removal,
    utils::removal::Removal,
};

use crate::miscfunc;
use crate::systems;

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

        miscfunc::init_camera(world, &dimensions, 0.5);

        println!("In-game state now active!");

        create_player(world, &dimensions);

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

pub fn create_player(world: &mut World, dimensions: &ScreenDimensions){

    let sprites = miscfunc::load_spritesheet(world, "sprites/player/player");

    let mut transform = Transform::default();
    transform.set_translation_xyz( dimensions.width() * 0.5, dimensions.height() * 0.5, 0.);

    let renderer = SpriteRender {
        sprite_sheet: sprites,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(renderer)
        .with(transform)
        .with(systems::entities::player_movement_system::PlayerMovement::new())
        .with(Removal::new(-1))
        .build();

}