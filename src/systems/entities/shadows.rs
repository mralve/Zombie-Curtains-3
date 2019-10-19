use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, transform::components::Parent, Transform},
    ecs::{prelude::*, Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent,
    },
};

pub struct Sprite_Shadow {
    pub sprite: Handle<SpriteSheet>,
}

#[derive(Default)]
pub struct E_Shadow {
    pub shadow_offset: f32,
}

impl Component for E_Shadow {
    type Storage = VecStorage<Self>;
}

impl E_Shadow {
    pub fn new() -> E_Shadow {
        E_Shadow { shadow_offset: 0.0 }
    }
}
// 1034684314
pub struct E_ShadowSystem;

impl<'s> System<'s> for E_ShadowSystem {
    type SystemData = (
        WriteStorage<'s, E_Shadow>,
        WriteExpect<'s, Sprite_Shadow>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut offsets, shadow_sprite, mut transforms, entities, lazy): Self::SystemData,
    ) {
        let renderer = SpriteRender {
            sprite_sheet: shadow_sprite.sprite.clone(),
            sprite_number: 0,
        };

        let mut vec = Vec::new();
        for (offset, transform, ents) in (&offsets, &mut transforms, &entities).join() {
            let mut finaltransform = Transform::default();
            finaltransform.set_translation_y(offset.shadow_offset);
            lazy.create_entity(&entities)
                .with(finaltransform)
                .with(Parent::new(ents))
                .with(renderer.clone())
                .with(Transparent)
                .build();
            vec.push(ents);
        }
        for i in vec {
            &offsets.remove(i);
        }
    }
}
