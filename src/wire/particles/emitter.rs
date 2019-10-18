use amethyst::{
    core::{timing::Time, Transform},
    ecs::{prelude::*, Entities, NullStorage},
    prelude::*,
    renderer::sprite::{SpriteRender, SpriteSheetHandle},
};

use crate::zombie_curtains::WorldResources;
extern crate rand;
use rand::prelude::*;

/// Basic property defining a particles max and min.
#[derive(Default)]
pub struct PropertyRange {
    pub max: f32,
    pub min: f32,
}

impl PropertyRange {
    pub fn new() -> PropertyRange {
        PropertyRange { min: 0., max: 40. }
    }
}

/// This is the essential particle
/// all particle effectors use this component to apply the effects.
#[derive(Default)]
pub struct ParticleEmitter {
    pub particle_count: f32,
    pub particle_life: PropertyRange,
    pub local_space: bool,
    pub particle_spawn_rate: f32,
    pub particle_spawn_amount: i32,
    pub temp_tracker_sr: f32,
    pub particle_start_rotation: PropertyRange,
    pub spawn_box_radius_x: PropertyRange,
    pub spawn_box_radius_y: PropertyRange,
}

impl ParticleEmitter {
    pub fn new() -> ParticleEmitter {
        ParticleEmitter {
            particle_count: 1000.0,
            particle_life: PropertyRange::new(),
            local_space: false,
            particle_spawn_rate: 1.,
            particle_spawn_amount: 2,
            temp_tracker_sr: 0.,
            particle_start_rotation: PropertyRange {
                min: -180.,
                max: 180.,
            },
            spawn_box_radius_x: PropertyRange { min: -5., max: 5. },
            spawn_box_radius_y: PropertyRange { min: -5., max: 5. },
        }
    }

    pub fn new_emitter(self, world: &mut World, start_transform: Transform) {
        world
            .create_entity()
            .with(self)
            .with(start_transform)
            .build();
    }
}

#[derive(Default)]
pub struct EmitterTracker {
    pub alive_particles: i32,
    pub cur_spawn_ratetime: f32,
}

impl EmitterTracker {
    pub fn new() -> EmitterTracker {
        EmitterTracker {
            alive_particles: 0,
            cur_spawn_ratetime: 0.,
        }
    }
}

impl Component for ParticleEmitter {
    type Storage = VecStorage<Self>;
}

impl Component for EmitterTracker {
    type Storage = VecStorage<Self>;
}

pub struct ParticleEmitterSystem;

impl<'s> System<'s> for ParticleEmitterSystem {
    type SystemData = (
        WriteStorage<'s, ParticleEmitter>,
        WriteStorage<'s, EmitterTracker>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut particle, mut particle_tracker, mut transforms, mut entities, time): Self::SystemData,
    ) {
        let mut particlesToAdd: Vec<Transform> = vec![];
        for (particle_comp, transform_comp) in (&mut particle, &mut transforms).join() {
            if particle_comp.temp_tracker_sr >= particle_comp.particle_spawn_rate {
                particle_comp.temp_tracker_sr = 0.;
                for _i in 0..particle_comp.particle_spawn_amount {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        transform_comp.translation().x,
                        transform_comp.translation().y,
                        transform_comp.translation().z,
                    );
                    particlesToAdd.push(transform);
                }
            }
            particle_comp.temp_tracker_sr += 1. * time.delta_seconds();
        }

        for transform in particlesToAdd {
            entities
                .build_entity()
                .with(transform, &mut transforms)
                .build();
        }
    }
}
