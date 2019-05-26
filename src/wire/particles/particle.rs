use amethyst::{
    core::{timing::Time, Float, Transform},
    ecs::{prelude::*, Entities, NullStorage},
    renderer::sprite::{SpriteRender, SpriteSheetHandle},
};

use crate::zombie_curtains::WorldResources;
extern crate rand;
use rand::prelude::*;

/// This is the essential particle
/// all particle effectors use this component to apply the effects.
#[derive(Default)]
pub struct Particle {
    pub par_life: f32,
    pub par_end: f32,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            par_life: 0.0,
            par_end: 6.0,
        }
    }

    pub fn new_ent(world: &mut World) {}
}

impl Component for Particle {
    type Storage = VecStorage<Self>;
}

pub struct ParticleLifeTimeSystem;

impl<'s> System<'s> for ParticleLifeTimeSystem {
    type SystemData = (WriteStorage<'s, Particle>, Entities<'s>, Read<'s, Time>);

    fn run(&mut self, (mut particle, ents, time): Self::SystemData) {
        for (particle_comp, cur_ent) in (&mut particle, &ents).join() {
            if particle_comp.par_life >= particle_comp.par_end {

            } else {

            }
        }
    }
}
