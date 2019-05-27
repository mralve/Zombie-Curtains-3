use crate::wire::particles::emitter::ParticleEmitterSystem;
use crate::wire::particles::particle::ParticleLifeTimeSystem;
use amethyst::{core::SystemBundle, ecs::prelude::DispatcherBuilder, error::Error};

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
        builder.add(ParticleLifeTimeSystem, "lifetime", &[]);
        builder.add(ParticleEmitterSystem, "particle_emitter", &[]);
        Ok(())
    }
}
