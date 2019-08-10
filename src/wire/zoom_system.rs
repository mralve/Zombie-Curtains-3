use amethyst::{
    core::{timing::Time, Transform},
    ecs::{prelude::*, Join, NullStorage, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, ScrollDirection, StringBindings},
    renderer::camera::{Camera, Orthographic},
    utils::ortho_camera::*,
    window::ScreenDimensions,
};

//use crate::wire::VelSlideComp;

#[derive(Default)]
pub struct ZoomComp {
    pub camera_scale: f32,
}

impl ZoomComp {
    pub fn new() -> ZoomComp {
        ZoomComp { camera_scale: 2.5 }
    }
}

impl Component for ZoomComp {
    type Storage = VecStorage<Self>;
}

pub struct ZoomSystem;

impl<'s> System<'s> for ZoomSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, ZoomComp>,
        WriteStorage<'s, CameraOrtho>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cam, dim, mut zoom, mut ortho, input, time): Self::SystemData) {
        let mut scroll: f64;
        let aspect = dim.aspect_ratio();
        for (cam, ortho_comp, zoom_comp) in (&mut cam, &mut ortho, &mut zoom).join() {
            scroll = input.mouse_wheel_value(false).into();
            if scroll != 0.0 {
                let scaled_amount = 10. * scroll as f32;
                zoom_comp.camera_scale += scaled_amount;

                ortho_comp.world_coordinates = CameraOrthoWorldCoordinates {
                    left: -1920. / zoom_comp.camera_scale,
                    right: 1920. / zoom_comp.camera_scale,
                    bottom: -1080. / zoom_comp.camera_scale,
                    top: 1080. / zoom_comp.camera_scale,
                };

                let offsets = ortho_comp.camera_offsets(aspect);

                let (near, far) = if let Some(prev) = cam.projection().as_orthographic() {
                    (prev.near(), prev.far())
                } else {
                    continue;
                };

                cam.set_projection(
                    Orthographic::new(offsets.0, offsets.1, offsets.2, offsets.3, near, far).into(),
                );
            }
        }
    }
}
