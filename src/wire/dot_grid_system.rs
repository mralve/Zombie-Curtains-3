use amethyst::{
    core::Float,
    ecs::prelude::*,
    ecs::NullStorage,
    core::Transform,
    renderer::{SpriteRender, SpriteSheetHandle}
};

#[derive(Default)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
}

impl Dot {
    pub fn new(pos: [2; i32]) -> Dot {
        Dot {
            x: pos.0,
            y: pos.1,
        }
    }
}

impl Component for MoveComp {
    type Storage = NullStorage<Self>;
}

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Entities<'s>,

    );

    fn run(&mut self, (entities): Self::SystemData) {
        for ents in (entities).join() {

        }
    }

}