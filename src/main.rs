#![windows_subsystem = "windows"]

mod renderer_graph;

mod systems;
mod wire;
mod zombie_curtains;

extern crate amethyst;

use amethyst::{
    assets::Processor,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{
        sprite::SpriteSheet, sprite_visibility::SpriteVisibilitySortingSystem,
        types::DefaultBackend, RenderingSystem,
    },
    ui::UiBundle,
    utils::ortho_camera::CameraOrthoSystem,
    window::DisplayConfig,
    window::WindowBundle,
};

use crate::renderer_graph::RendererGraph;
//use crate::systems::systems_bundle;
use crate::wire::particles::particles_bundle::ParticlesBundle;
use crate::wire::wire_editor_bundle::WireEditorBundle;
use crate::zombie_curtains::ZombieCurtains;
//use crate::wire::editor_state::WireEditor;

pub const WIDTH: &'static f32 = &(1920. / 1.5);
pub const HEIGHT: &'static f32 = &(1080. / 1.5);

fn main() -> amethyst::Result<()> {
    //amethyst::start_logger(Default::default());
    let mut config = DisplayConfig::default();
    config.title = "Zombie Curtains 3".to_string();
    config.decorations = true;
    config.resizable = true;
    config.transparent = false;
    config.dimensions = Some((*WIDTH as u32, *HEIGHT as u32));


    use amethyst::input::{InputBundle, StringBindings};
    use amethyst::utils::application_dir;
    let binding_path = application_dir("resources")?.join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(&binding_path)?;

    println!("{:?}", &binding_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config(config))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with_bundle(systems::GameSystemBundle)?
        .with_bundle(WireEditorBundle::new())?
        .with_bundle(ParticlesBundle::new())?
        
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RendererGraph::default(),
        ));

    let mut game = Application::build("./", ZombieCurtains)?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 999)
        .build(game_data)?;

    game.run();

    Ok(())
}
