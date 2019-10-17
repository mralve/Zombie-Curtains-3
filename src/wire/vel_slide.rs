use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Component, Join, Read, System, VecStorage, WriteStorage},
};

pub struct VelSlideSystem;

impl<'s> System<'s> for VelSlideSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, VelSlideComp>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut movement_comp, time): Self::SystemData) {
        for (movement, transform) in (&mut movement_comp, &mut transforms).join() {
            let obj_x = transform.translation().x;
            movement.velocity[0] *= 1. - (movement.drag * time.delta_seconds());
            if movement.velocity[0].abs() <= 0.01 {
                movement.velocity[0] = 0.;
            }

            transform.set_translation_x(obj_x + movement.velocity[0] as f32 * time.delta_seconds());

            let obj_y = transform.translation().y;
            movement.velocity[1] *= 1. - (movement.drag * time.delta_seconds());

            if movement.velocity[1].abs() <= 0.01 {
                movement.velocity[1] = 0.;
            }

            transform.set_translation_y(obj_y + movement.velocity[1] * time.delta_seconds());
        }
    }
}

pub struct VelSlideComp {
    pub velocity: [f32; 2],
    pub drag: f32,
}

impl VelSlideComp {
    pub fn new() -> VelSlideComp {
        VelSlideComp {
            velocity: [0., 0.],
            drag: 5.,
        }
    }
}

impl Component for VelSlideComp {
    type Storage = VecStorage<Self>;
}
