use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::Resources;
use amethyst::prelude::*;
use amethyst::renderer::WindowMessages;
use amethyst::renderer::{
    Camera, PngFormat, Projection, Renderer, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::ui::*;
use amethyst::utils::application_dir;
use amethyst::utils::ortho_camera::*;
use amethyst::{
    input::{is_close_requested, is_key_down, InputBundle},
    winit::VirtualKeyCode,
};

pub struct ZombieCurtains;

pub struct WorldResources {
    pub world_sprites: Vec<SpriteSheetHandle>,
}

use crate::systems::*;

pub const CAMERA_ZOOM: f32 = 2.5;
pub const CAMERA_SCALE_HEIGHT: f32 = 1080. / CAMERA_ZOOM;
pub const CAMERA_SCALE_WIDTH: f32 = 1920. / CAMERA_ZOOM;

impl SimpleState for ZombieCurtains {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);

        let world_sprites = vec![
            load_sprite_sheet(
                world,
                application_dir("resources")
                    .unwrap()
                    .join("textures")
                    .join("grass")
                    .to_string_lossy()
                    .to_string(),
                "tile".to_string(),
            ),
            load_sprite_sheet(
                world,
                application_dir("resources")
                    .unwrap()
                    .join("textures")
                    .join("grass2")
                    .to_string_lossy()
                    .to_string(),
                "tile".to_string(),
            ),
        ];

        world.add_resource(WorldResources {
            world_sprites: world_sprites,
        });

        world
            .create_entity()
            .with(GenerateChunk::new((0, 0)))
            .build();

        world
            .create_entity()
            .with(GenerateChunk::new((-1, 0)))
            .build();

        world
            .create_entity()
            .with(GenerateChunk::new((0, -1)))
            .build();

        world
            .create_entity()
            .with(GenerateChunk::new((-1, -1)))
            .build();
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
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

    use crate::editor_systems::MoveComp;
    // Editor movement

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            crate::WIDTH * 0.32 * 1.5,
            0.0,
            crate::HEIGHT * 0.32 * 1.5,
        )))
        .with(MoveComp::new())
        .with(transform)
        .with(camera_ortho)
        .build();
}

fn load_sprite_sheet(world: &mut World, path: String, ron: String) -> SpriteSheetHandle {
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

    let binding_path = application_dir("resources")
        .unwrap()
        .join("rons")
        .join(format!("{}.ron", ron));

    loader.load(
        binding_path.to_string_lossy(),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage,
    )
}
