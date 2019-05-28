use crate::systems::{entities::*, fps_system::Text, *};
use crate::wire::particles::{EmitterTracker, ParticleEmitter};

use amethyst::{
    assets::{AssetStorage, Loader},
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
    ui::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    utils::{application_dir, ortho_camera::*},
    winit::VirtualKeyCode,
};

pub struct ZombieCurtains;

// TODO: A multi state world resources loading is needed!
pub struct WorldResources {
    pub world_sprites: Vec<SpriteSheetHandle>,
    pub entity_sprites: Vec<SpriteSheetHandle>,
}

pub const CAMERA_ZOOM: f32 = 2.5;
pub const CAMERA_SCALE_HEIGHT: f32 = 1080. / CAMERA_ZOOM;
pub const CAMERA_SCALE_WIDTH: f32 = 1920. / CAMERA_ZOOM;

impl SimpleState for ZombieCurtains {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);
        initialize_debugui(world);

        let world_sprites = vec![
            load_sprite(
                world,
                application_dir("resources")
                    .unwrap()
                    .join("textures")
                    .join("grass")
                    .to_string_lossy()
                    .to_string(),
                "tile".to_string(),
            ),
            load_sprite(
                world,
                application_dir("resources")
                    .unwrap()
                    .join("textures")
                    .join("flower")
                    .to_string_lossy()
                    .to_string(),
                "tile".to_string(),
            ),
            load_sprite(
                world,
                application_dir("resources")
                    .unwrap()
                    .join("textures")
                    .join("particle")
                    .to_string_lossy()
                    .to_string(),
                "tile".to_string(),
            ),
        ];

        let entity_sprites = vec![load_sprite(
            world,
            application_dir("resources")
                .unwrap()
                .join("textures")
                .join("wab_player")
                .to_string_lossy()
                .to_string(),
            "player".to_string(),
        )];
        let sprite = SpriteRender {
            sprite_sheet: entity_sprites[0].clone(),
            sprite_number: 0,
        };

        world.add_resource(WorldResources {
            world_sprites: world_sprites,
            entity_sprites: entity_sprites,
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
            .with(PlayerMovement::new())
            .with(Transform::default())
            .with(ParticleEmitter::new())
            .with(sprite)
            .build();

        //world.create_entity().with(Transform::default()).build();
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
            StateEvent::Ui(_ui_event) => Trans::None,
            StateEvent::Input(_input) => Trans::None,
        }
    }
}

/*
fn init_textdebug(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        application_dir("resources")
            .unwrap()
            .join("font")
            .join("MontserratSB.ttf")
            .to_string_lossy()
            .to_string(),
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let fps_transform = UiTransform::new(
        "fps".to_string(),
        Anchor::TopLeft,
        Anchor::MiddleLeft,
        500.,
        -25.,
        1.,
        100.,
        50.,
    );

    let fps_text = world
        .create_entity()
        .with(fps_transform)
        .with(UiText::new(
            font.clone(),
            "I AM THE ZERO FPS, HO NO!".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.,
        ))
        .build();

    world.add_resource(Text { fps: fps_text });
}
*/

/// Initialises a ui scoreboard
fn initialize_debugui(world: &mut World) {
    // Create a simple initial interface for rendering ui

    let font = world.read_resource::<Loader>().load(
        application_dir("resources")
            .unwrap()
            .join("font")
            .join("MontserratSB.ttf")
            .to_string_lossy()
            .to_string(),
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let engine = UiTransform::new(
        "we".to_string(),
        Anchor::TopLeft,
        Anchor::Middle,
        104.,
        -10.,
        1.,
        200.,
        50.,
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopLeft,
        Anchor::Middle,
        104.,
        -25.,
        1.,
        200.,
        50.,
    );

    let dt_transform = UiTransform::new(
        "dt".to_string(),
        Anchor::TopLeft,
        Anchor::Middle,
        104.,
        -40.,
        1.,
        200.,
        50.,
    );

    let mut font = UiText::new(
        font.clone(),
        "Debug info".to_string(),
        [1., 1., 1., 1.],
        15.,
    );

    font.align = Anchor::MiddleLeft;

    world
        .create_entity()
        .with(engine)
        .with(font.clone())
        .build();

    font.color = [0., 1., 0., 1.];
    let fps = world
        .create_entity()
        .with(p1_transform)
        .with(font.clone())
        .build();

    font.color = [1., 0.6, 0.0, 0.8];

    let dt = world.create_entity().with(dt_transform).with(font).build();

    world.add_resource(Text { fps });
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

    transform.set_translation_z(3.0);

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
            10.0,
        )))
        //       .with(MoveComp::new())
        //       .with(VelSlideComp::new())
        .with(transform)
        .with(CameraMovement::new())
        //.with(GeneratorSource::new())
        .with(ZoomComp::new())
        .with(camera_ortho)
        .build();
}

#[derive(Debug, Clone)]
struct LoadedSpriteSheet {
    sprite_sheet_handle: SpriteSheetHandle,
    sprite_count: u32,
    sprite_w: u32,
    sprite_h: u32,
}

/*
fn load_sprite_sheet(world: &mut World) -> LoadedSpriteSheet {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/arrow_semi_transparent.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let sprite_w = 32;
    let sprite_h = 32;
    let sprite_sheet_definition = SpriteSheetDefinition::new(sprite_w, sprite_h, 2, 6, false);

    let sprite_sheet = sprite_sheet_loader::load(texture_handle, &sprite_sheet_definition);
    let sprite_count = sprite_sheet.sprites.len() as u32;

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprite_sheet,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };

    LoadedSpriteSheet {
        sprite_sheet_handle,
        sprite_count,
        sprite_w,
        sprite_h,
    }
}
*/

fn load_sprite(world: &mut World, path: String, ron: String) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    let ronpath = application_dir("resources")
        .unwrap()
        .join("rons")
        .join(format!("{}.ron", ron));

    loader.load(
        ronpath.to_string_lossy(),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
