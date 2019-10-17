use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::ui::*;
use amethyst::utils::removal::*;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader},
    ecs::World,
    renderer::{loaders::load_from_srgba, palette::Srgba, types::TextureData, Texture},
};

/// Block is a fundamental ui object, it inherits a color and border_block properties
pub struct Block {
    /// Blocks background color.
    pub color: [f32; 4],
    /// Blocks Borders
    pub border: BorderBlock,
}

impl Block {
    /// Creates new UiBlock
    pub fn new(color: [f32; 4], border: BorderBlock) -> Block {
        Block {
            color: color,
            border: border,
        }
    }
}

/// BorderBlock
pub struct BorderBlock {
    /// Border's radius, this set's the amount of rounding each of the border's corners will receive.
    pub border_radius: [f32; 4],
    /// Set's each border's width
    pub border: [f32; 4],
}

/// Decides what the border will be placed on the block.
pub enum BorderSeat {
    Center,
    Inside,
    Outside,
}

///
pub enum BorderEdgeSeat {
    Connected,
    Individual,
}

/// Creates a basic block
pub fn wire_create_basic_block(
    world: &mut amethyst::prelude::World,
    bg_color: [f32; 4],
    transform: UiTransform,
    removal_id: i16,
) -> Entity {
    let texture_handle: Handle<Texture>;
    {
        let loader = world.read_resource::<Loader>();
        let texture_assets = world.read_resource::<AssetStorage<Texture>>();
        let texture_builder = load_from_srgba(Srgba::new(
            bg_color[0],
            bg_color[1],
            bg_color[2],
            bg_color[3],
        ));
        texture_handle =
            loader.load_from_data(TextureData::from(texture_builder), (), &texture_assets);
    }

    return world
        .create_entity()
        .with(texture_handle)
        .with(transform)
        .with(Interactable)
        .with(Removal::new(removal_id))
        .build();
}

/*
/// Creates a simple image from data.
pub fn create_image(world: &mut amethyst::prelude::World, image: String, meta: TextureMetadata, transform: UiTransform, removal_id: i16) -> Entity
{

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            image,
            PngFormat,
            meta,
            (),
            &texture_storage,
        )
    };

    let removal_comp = Removal::new(removal_id);

    return world.create_entity()
        .with(texture_handle)
        .with(transform)
        .with(removal_comp)
        .build();
}

*/
/// Removes all ui with the certain ID
pub fn remove_ui(world: &mut amethyst::prelude::World, ids: i16) {
    exec_removal(&world.entities(), &world.read_storage(), ids)
}
