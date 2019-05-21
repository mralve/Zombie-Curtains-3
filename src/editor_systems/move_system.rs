use amethyst::core::Float;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::core::Transform;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

#[derive(Default)]
pub struct MoveComp;

impl MoveComp {
    pub fn new() -> MoveComp {
        MoveComp {

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

    }

}