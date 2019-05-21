use amethyst::error::Error;
use amethyst::utils::ortho_camera;
use amethyst::{core::SystemBundle, ecs::prelude::DispatcherBuilder};


use crate::systems;

///
/// Systems bundle, the bundle for the Zombie Curtains game systems
/// 

#[derive(Default)]
pub struct GameSystemBundle;

impl GameSystemBundle {
    /// Create a new transform bundle
    pub fn new() -> Self {
        GameSystemBundle {

        }
    }

}

impl<'a, 'b> SystemBundle<'a, 'b> for GameSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            systems::ChunkGeneratorSystem,
            "game_chunk_generator",
            &[]
        );
        builder.add(ortho_camera::CameraOrthoSystem, "camera_ortho_system", &[]);
        Ok(())
    }
}
