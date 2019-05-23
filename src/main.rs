//#![windows_subsystem = "windows"]

mod render_graph;

mod wire;
mod particles;
mod systems;
mod zombie_curtains;

extern crate amethyst;

use amethyst::{
    prelude::*,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    window::{DisplayConfig},
    renderer::{types::DefaultBackend, RenderingSystem, transparent::Transparent},
    ui::{DrawUi, UiBundle},
    utils::ortho_camera::CameraOrthoSystem,
    window::WindowBundle,
};

use crate::wire::wire_editor_bundle::WireEditorBundle;
use crate::render_graph::RenderGraph;
use crate::systems::systems_bundle;
use crate::zombie_curtains::ZombieCurtains;

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
    let Input_Bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(&binding_path)?;

    println!("{:?}", &binding_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config(config))?
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(RenderGraph::default(),))
        .with_bundle(TransformBundle::new())?
        .with_bundle(Input_Bundle)?
        .with_bundle(systems::GameSystemBundle)?
        .with_bundle(particles::particles_bundle::ParticlesBundle)?
        .with_bundle(WireEditorBundle::new())?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with(CameraOrthoSystem, "orthographic_camera", &[]);

    let mut game = Application::build("./", ZombieCurtains)?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 999)
        .build(game_data)?;

    game.run();

    Ok(())
}
