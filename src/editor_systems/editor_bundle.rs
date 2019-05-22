use amethyst::error::Error;
use amethyst::{core::SystemBundle, ecs::prelude::DispatcherBuilder};

use crate::editor_systems::MoveSystem;
use crate::editor_systems::VelSlideSystem;
use crate::editor_systems::ZoomSystem;

///
/// Editor Bundle, use full systems for a in-game editor.
///

#[derive(Default)]
pub struct EditorBundle;

impl EditorBundle {
    /// Create a new transform bundle
    pub fn new() -> Self {
        EditorBundle {}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for EditorBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(MoveSystem, "editor_move_system", &[]);
        builder.add(VelSlideSystem, "editor_vel_slide_system", &[]);
        builder.add(ZoomSystem, "editor_zoom_system", &[]);
        Ok(())
    }
}
