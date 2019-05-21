use amethyst::core::Float;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::core::Transform;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use crate::zombie_curtains::WorldResources;

pub struct Chunk {
    pub tile_start_id: i32,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            tile_start_id: 0,
        }
    }
}

impl Component for Chunk {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct GenerateChunk {
    pub x: i32,
    pub y: i32,
    pub generated: bool,
}

impl GenerateChunk {
    pub fn new(pos: (i32, i32)) -> GenerateChunk {
        GenerateChunk {
            x: pos.0,
            y: pos.1,
            generated: false,
        }
    }
}

impl Component for GenerateChunk {
    type Storage = VecStorage<Self>;
}

pub struct ChunkGeneratorSystem;

impl<'s> System<'s> for ChunkGeneratorSystem {
    type SystemData = (
        WriteStorage<'s, GenerateChunk>,
        Entities<'s>,
        ReadExpect<'s, WorldResources>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut chunk, entities, resources, mut transforms, mut sprites): Self::SystemData) {
        for chunk_comp in (&mut chunk).join() {
            if !chunk_comp.generated {
                chunk_comp.generated = true;
                for y in 0..16 {
                    for x in 0..16 {
                        let mut transform = Transform::default();
                        let sprite = SpriteRender {
                            sprite_sheet: resources.world_sprites.clone(),
                            sprite_number: 0,
                        };

                        //transform.set_translation_xyz(Float::from(chunk_comp.x as f32 * 512. + x as f32 * 32.), Float::from(chunk_comp.y as f32 * 512. + y as f32 * 32.), 0.);
                        
                        let tile_x = chunk_comp.x * 512 + x * 32;
                        let tile_y = chunk_comp.y * 512 + y * 32;
                        transform.set_translation_xyz(Float::from(tile_x as f32), Float::from(tile_y as f32), 0.);

                        entities.build_entity()
                            .with(transform, &mut transforms)
                            .with(sprite, &mut sprites)
                            .build();

                    }
                }
            }
        }
    }

}