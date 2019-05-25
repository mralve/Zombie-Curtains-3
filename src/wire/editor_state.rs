//////////////////////////////////////////////////////////////////
//                                                              //
//    Wire Editor, experimental editor system inside of ECS.    //
//                                                              //
//                                                              //
//                                                              //
//////////////////////////////////////////////////////////////////

use crate::systems::{entities::*, *};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
    utils::{application_dir, ortho_camera::*},
    winit::VirtualKeyCode,
};

pub const CAMERA_ZOOM: f32 = 2.5;
pub const CAMERA_SCALE_HEIGHT: f32 = 1080. / CAMERA_ZOOM;
pub const CAMERA_SCALE_WIDTH: f32 = 1920. / CAMERA_ZOOM;

pub struct WireEditor;

impl SimpleState for WireEditor {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    // TODO: Add a warning to the user to save potential the changes.
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => Trans::None,
            StateEvent::Input(input) => Trans::None,
        }
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    let mut camera_ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: -CAMERA_SCALE_WIDTH / 2.,
        right: CAMERA_SCALE_WIDTH / 2.,
        bottom: -CAMERA_SCALE_HEIGHT / 2.,
        top: CAMERA_SCALE_HEIGHT / 2.,
    };

    transform.set_translation_z(2.0);

    use crate::wire::{MoveComp, VelSlideComp, ZoomComp};
    // Editor movement

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            crate::WIDTH * 0.32 * 1.5,
            0.0,
            crate::HEIGHT * 0.32 * 1.5,
            0.0,
            5.0,
        )))
        .with(MoveComp::new())
        .with(VelSlideComp::new())
        .with(transform)
        .with(GeneratorSource::new())
        .with(camera_ortho)
        .with(ZoomComp::new())
        .build();
}
