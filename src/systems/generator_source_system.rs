use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::systems::*;

#[derive(Default)]
pub struct GeneratorSource;

impl GeneratorSource {
    pub fn new() -> GeneratorSource {
        GeneratorSource
    }
}

impl Component for GeneratorSource {
    type Storage = NullStorage<Self>;
}

pub struct GeneratorSourceSystem;

impl<'s> System<'s> for GeneratorSourceSystem {
    type SystemData = (
        ReadStorage<'s, GeneratorSource>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, GenerateChunk>,
        Entities<'s>,
    );

    fn run(&mut self, (generator_sources, transforms, mut chunks, entities): Self::SystemData) {
        let mut x = 0;
        let mut y = 0;

        for (_generator_source, transform) in (&generator_sources, &transforms).join() {
            let (chunk_x, chunk_y) = {
                let mut x = (transform.translation().x.as_f32() / 512.) as i32;
                let mut y = (transform.translation().y.as_f32() / 512.) as i32;

                if transform.translation().x.as_f32() < 0. {
                    x -= 1
                };
                if transform.translation().y.as_f32() < 0. {
                    y -= 1
                };

                (x, y)
            };

            x = chunk_x;
            y = chunk_y;
        }

        for iy in -2..2 {
            for ix in -2..2 {
                if !check_chunk(x - ix, y - iy, &mut chunks) {
                    //println!("Do, x: {}, y: {}", x - ix, y - iy);
                    entities
                        .build_entity()
                        .with(GenerateChunk::new((x - ix, y - iy)), &mut chunks)
                        .build();
                } else {
                    //println!("Don't, x: {}, y: {}", x - ix, y - iy);
                }
            }
        }
    }
}

fn check_chunk(x: i32, y: i32, chunks: &mut WriteStorage<GenerateChunk>) -> bool {
    for chunk in chunks.join() {
        if chunk.x == x && chunk.y == y {
            return true;
        }
    }
    false
}
