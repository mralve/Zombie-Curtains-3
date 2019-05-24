use crate::particles;
use amethyst::{
    core::SystemBundle, ecs::prelude::DispatcherBuilder, error::Error, utils::ortho_camera,
};

///
/// ParticlesBundle, WIP
///

#[derive(Default)]
pub struct ParticlesBundle;

impl ParticlesBundle {
    /// Create a new transform bundle
    pub fn new() -> Self {
        ParticlesBundle {}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for ParticlesBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        //builder.add(systems::ChunkGeneratorSystem, "part_system", &[]);
        Ok(())
    }
}
