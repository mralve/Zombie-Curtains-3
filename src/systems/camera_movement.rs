use amethyst::{
    core::{
        math::{ComplexField, RealField},
        timing::Time,
        Transform,
    },
    ecs::{Component, Join, Read, ReadStorage, System, VecStorage, WriteStorage},
};

use crate::systems::entities::*;

#[derive(Default)]
pub struct CameraMovement {
    butter: f32,
}

impl CameraMovement {
    pub fn new() -> CameraMovement {
        CameraMovement { butter: 4. }
    }
}

impl Component for CameraMovement {
    type Storage = VecStorage<Self>;
}

pub struct CameraMovementSystem;

/// Moves the camera to the desired entity that has PlayerMovement component.
/// Add CameraMovement comp to a camera entity to make the camera start folowing the player.
///
impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, CameraMovement>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PlayerMovement>,
        Read<'s, Time>,
    );

    fn run(&mut self, (camera_movements, mut transforms, players, time): Self::SystemData) {
        let mut px = 0.;
        let mut py = 0.;

        for (_player, transform) in (&players, &transforms).join() {
            px = transform.translation().x;
            py = transform.translation().y;
        }

        for (camera_movement, transform) in (&camera_movements, &mut transforms).join() {
            let (x, y) = (transform.translation().x, transform.translation().y);
            let dx = px - x;
            let dy = py - y;
            let dir = dy.atan2(dx);
            let dist = ((dx * dx + dy * dy).sqrt()).abs();

            transform.append_translation_xyz(
                dir.cos() * dist * camera_movement.butter * time.delta_seconds(),
                dir.sin() * dist * camera_movement.butter * time.delta_seconds(),
                0.,
            );
        }
    }
}
