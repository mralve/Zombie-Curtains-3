use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    ecs::{prelude::*, Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(Default)]
pub struct SpriteFlipper {
    pub last_x: f32,
    pub distance_trigger: f32,
}

impl Component for SpriteFlipper {
    type Storage = VecStorage<Self>;
}

impl SpriteFlipper {
    pub fn new() -> SpriteFlipper {
        SpriteFlipper {
            last_x: 0.0,
            distance_trigger: 0.0,
        }
    }
}
// 1034684314
pub struct SpriteFlipperSystem;

impl<'s> System<'s> for SpriteFlipperSystem {
    type SystemData = (
        WriteStorage<'s, SpriteFlipper>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut flippers, mut transforms, time): Self::SystemData) {
        let delta = time.delta_seconds();
        for (flipper, transform) in (&mut flippers, &mut transforms).join() {
            if flipper.distance_trigger >= 0.05 {
                flipper.distance_trigger = 0.0;

                let cur_x = transform.translation().x;

                if flipper.last_x > cur_x - flipper.distance_trigger {
                    transform.set_scale(Vector3::new(-1.0, 1.0, 1.0));
                } else if flipper.last_x < cur_x + flipper.distance_trigger {
                    transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
                }

                // Sample the current x after calc on the one before.
                flipper.last_x = cur_x;
            }
            flipper.distance_trigger += 1.0 * delta;
        }
    }
}
