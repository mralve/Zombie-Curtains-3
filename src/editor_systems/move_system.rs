use amethyst::core::Float;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

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
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MoveComp>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transform, mov, input, time): Self::SystemData) {
        /*
         */
        for (trans, movement_comp) in (&mut transform, &mov).join() {
            let movement_ud = input.axis_value("ud");
            let movement_lr = input.axis_value("lr");

            if movement_lr != Some(0.) {
                let scaled_amount = 200. * movement_lr.unwrap() as f32;
                &trans.prepend_translation_x(scaled_amount * time.delta_seconds());
            }

            if movement_ud != Some(0.) {
                let scaled_amount = 200. * movement_ud.unwrap() as f32;
                &trans.prepend_translation_y(scaled_amount * time.delta_seconds());
            }
        }
    }
}