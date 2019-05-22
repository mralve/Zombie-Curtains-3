use amethyst::core::Float;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use crate::zombie_curtains::WorldResources;
extern crate rand;
use rand::prelude::*;

#[derive(Deafult)]
pub struct Particle {
    life: f32;
}

impl Particle {
    pub fn new() -> Particle {
        Particle { 0 }
    }
}

impl Component for Particle {
    type Storage = VecStorage<Self>;
}

pub struct ParticleLifeTimeSystem;

impl<'s> System<'s> for ParticleLifeTimeSystem {
    type SystemData = (
        WriteStorage<'s, GenerateChunk>,
        Entities<'s>,
        ReadExpect<'s, WorldResources>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (mut chunk, entities, resources, mut transforms, mut sprites): Self::SystemData,
    ) {

    }
}