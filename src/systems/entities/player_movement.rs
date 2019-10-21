use amethyst::{
    core::{timing::Time, Transform},
    ecs::{prelude::*, Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(Default)]
pub struct PlayerMovement {
    speed: f32,
}

impl Component for PlayerMovement {
    type Storage = VecStorage<Self>;
}

impl PlayerMovement {
    pub fn new() -> PlayerMovement {
        PlayerMovement { speed: 1.2 }
    }
}

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, PlayerMovement>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (player_movements, mut transforms, time, input): Self::SystemData) {
        for (player_movement, transform) in (&player_movements, &mut transforms).join() {
            let (in_x, in_y) = (input.axis_value("lr"), input.axis_value("ud"));
            let (move_x, move_y) = {
                if in_x.is_some() && in_y.is_some() {
                    (
                        in_x.unwrap() as f32 * player_movement.speed * time.delta_seconds() * 100.,
                        in_y.unwrap() as f32 * player_movement.speed * time.delta_seconds() * 100.,
                    )
                } else {
                    (0., 0.)
                }
            };

            transform.append_translation_xyz(move_x, move_y, 0.);
        }
    }
}
