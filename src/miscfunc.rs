use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    utils::removal::Removal,
};

use crate::systems;

/// This creates the in-game camera.
pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions, zoom: f32) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 2.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width() * zoom, dimensions.height() * zoom))
        .with(transform)
        .with(systems::camera_movement_system::CameraMovement::new())
        .with(Removal::new(-1))
        .build();
}

pub fn load_spritesheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}{}", path, ".png"),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
 
    // Load the SpriteSheet definition file, which contains metadata on our
    // SpriteSheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}{}", path, ".ron"),
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };
    sheet_handle
}

pub fn load_texture(world: &mut World, path: &str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    return loader.load(
        format!("{}{}", path, ".png"),
        ImageFormat::default(),
        (),
        &texture_storage,
    )
}

/// Just creates a sprite at the given pos
pub fn init_sprite(world: &mut World, sprite: Handle<SpriteSheet>, target_sprite: usize, x_offset: f32, y_offset: f32, z_offset: f32 ) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x_offset, y_offset, z_offset);

    let renderer = SpriteRender {
        sprite_sheet: sprite,
        sprite_number: target_sprite,
    };

    world
        .create_entity()
        .with(renderer)
        .with(transform)
        .with(Removal::new(-1))
        .build();
}