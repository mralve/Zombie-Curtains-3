use amethyst::core::{timing::Time, Float, Transform};
use amethyst::ecs::{prelude::*, Join, NullStorage, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::input::ScrollDirection;

use amethyst::renderer::Camera;
use amethyst::utils::ortho_camera::*;

use crate::editor_systems::VelSlideComp;

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
        ReadStorage<'s, Camera>,
        WriteStorage<'s, ZoomComp>,
        WriteStorage<'s, CameraOrtho>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut zoom, mut ortho, input, time): Self::SystemData) {
        /*
         */
        let mut scroll: Option<f64>;
        for (ortho_comp, zoom_comp, camera_comp) in (&mut ortho, &mut zoom, &cameras).join() {
            //scroll = input.axis_value("scroll");
/*

            if input.mouse_button_is_down() {
                let scaled_amount = 1000. * scroll.unwrap() as f32;
                zoom_comp.camera_scale += scaled_amount * time.delta_seconds();

                ortho_comp.world_coordinates = CameraOrthoWorldCoordinates {
                    left: -1920. / zoom_comp.camera_scale,
                    right: 1920. / zoom_comp.camera_scale,
                    bottom: -1080. / zoom_comp.camera_scale,
                    top: 1080. / zoom_comp.camera_scale,
                }
            }
    */
        }
    }
}
