use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Resources;
use amethyst::prelude::*;
use amethyst::renderer::{
    Renderer, Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};
use amethyst::ui::*;
use amethyst::renderer::WindowMessages;
use amethyst::{
    input::{
        is_close_requested,
        is_key_down,
        InputBundle,
    },
    winit::VirtualKeyCode,

};
use amethyst::utils::ortho_camera::*;

pub struct ZombieCurtains;

pub const CAMERA_SCALE_HEIGHT: f32 = 1080.;
pub const CAMERA_SCALE_WIDTH: f32 = 1920.;

impl SimpleState for ZombieCurtains {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<CameraOrtho>();
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

                    // TODO: Add a prompt to warn the use to save the content before exiting.

                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                Trans::None
            }
            StateEvent::Input(input) => {
                Trans::None
            }
        }
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    let mut camera_ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: 0.,
        right: CAMERA_SCALE_WIDTH,
        bottom: 0.,
        top: CAMERA_SCALE_HEIGHT,
    };

    transform.set_translation_z(2.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            crate::WIDTH * 1.,
            0.0,
            crate::HEIGHT * 1.,
        )))
        .with(camera_ortho)
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, path: String) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage,
    )
}
