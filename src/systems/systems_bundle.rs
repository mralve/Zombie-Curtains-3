use amethyst::error::Error;
use amethyst::utils::ortho_camera;
use amethyst::{core::SystemBundle, ecs::prelude::DispatcherBuilder};

use crate::systems;
use crate::systems::entities;

///
/// Systems bundle, the bundle for the Zombie Curtains game systems
///

#[derive(Default)]
pub struct GameSystemBundle;

impl GameSystemBundle {
    /// Create a new transform bundle
    pub fn new() -> Self {
        GameSystemBundle {}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(systems::ChunkGeneratorSystem, "game_chunk_generator", &[]);
        builder.add(ortho_camera::CameraOrthoSystem, "camera_ortho_system", &[]);
        builder.add(
            entities::player_movement_system::PlayerMovementSystem,
            "player_movement_system",
            &[],
        );
        builder.add(systems::CameraMovementSystem, "camera_movement_system", &[]);
        builder.add(
            systems::GeneratorSourceSystem,
            "generator_source_system",
            &[],
        );
        builder.add(
            systems::FPSSystem,
            "fps_system",
            &[],
        );
        Ok(())
    }
}
