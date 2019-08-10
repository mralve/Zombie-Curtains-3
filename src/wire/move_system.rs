use amethyst::{
    core::{timing::Time, Transform},
    ecs::prelude::*,
    ecs::NullStorage,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::wire::VelSlideComp;

#[derive(Default)]
pub struct MoveComp;

impl MoveComp {
    pub fn new() -> MoveComp {
        MoveComp {}
    }
}

impl Component for MoveComp {
    type Storage = NullStorage<Self>;
}

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, VelSlideComp>,
        ReadStorage<'s, MoveComp>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut vel, mov, input, time): Self::SystemData) {
        for (vel_comp, _movement_comp) in (&mut vel, &mov).join() {
            let movement_ud = input.axis_value("ud");
            let movement_lr = input.axis_value("lr");

            if movement_lr != Some(0.) {
                let scaled_amount = 2000. * movement_lr.unwrap() as f32;
                vel_comp.velocity[0] += scaled_amount * time.delta_seconds();
            }

            if movement_ud != Some(0.) {
                let scaled_amount = 2000. * movement_ud.unwrap() as f32;
                vel_comp.velocity[1] += scaled_amount * time.delta_seconds();
            }
        }
    }
}
