//#![windows_subsystem = "windows"]
extern crate amethyst;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::{
    ortho_camera::CameraOrthoSystem,
};


pub const WIDTH: &'static f32 = &(1920. / 1.5);
pub const HEIGHT: &'static f32 = &(1080. / 1.5);

mod zombie_curtains;
mod systems;
mod editor_systems;

use crate::systems::systems_bundle;
use crate::editor_systems::editor_bundle;
use crate::zombie_curtains::ZombieCurtains;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let mut config = DisplayConfig::default();
    config.title = "Zombie Curtains 3".to_string();
    config.decorations = true;
    config.resizable = true;
    config.transparent = false;
    config.fullscreen = false;
    config.dimensions = Some((*WIDTH as u32, *HEIGHT as u32));
    config.vsync = true;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            //.clear_target([0.0, 0.0, 0.0, 1.], 1.)
            .with_pass(DrawFlat2D::new().with_transparency_settings(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()),
    );

    use amethyst::input::InputBundle;
    use amethyst::utils::application_dir;
    let binding_path = application_dir("resources")?.join("bindings.ron");
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(&binding_path)?;

    println!("{:?}", &binding_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(systems::GameSystemBundle)?
        .with_bundle(editor_bundle::EditorBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(CameraOrthoSystem, "orthographic_camera", &[]);
        

    let mut game = Application::build("./", ZombieCurtains)?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 999)
        .build(game_data)?;

    game.run();

    Ok(())
}
