use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, timing::Time, Transform},
    ecs::{prelude::*, Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteSheet,
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
    type SystemData = (ReadStorage<'s, E_Shadow>, WriteStorage<'s, Transform>);

    fn run(&mut self, (offsets, mut transforms): Self::SystemData) {
        //let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        //for (offset, transform) in (&offsets, &mut transforms).join() {}
    }
}
