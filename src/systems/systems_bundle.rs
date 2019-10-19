use crate::systems;
use crate::systems::entities;
use amethyst::{
    core::{
        ecs::{DispatcherBuilder, World},
        SystemBundle,
    },
    error::Error,
    utils::ortho_camera,
};

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
    fn build(
        mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        //builder.add(systems::ChunkGeneratorSystem, "game_chunk_generator", &[]);
        //builder.add(systems::GeneratorSourceSystem, "generator_source_system", &[]);
        builder.add(ortho_camera::CameraOrthoSystem, "camera_ortho_system", &[]);
        builder.add(
            entities::player_movement::PlayerMovementSystem,
            "player_movement_system",
            &[],
        );
        builder.add(systems::CameraMovementSystem, "camera_movement_system", &[]);
        builder.add(
            systems::entities::SpriteFlipperSystem,
            "sprite_flipper_system",
            &[],
        );
        //builder.add(systems::FPSSystem, "fps_text", &[]);

        Ok(())
    }
}
