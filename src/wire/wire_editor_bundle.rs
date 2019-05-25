use amethyst::{core::SystemBundle, ecs::prelude::DispatcherBuilder, error::Error};

use crate::wire::MoveSystem;
use crate::wire::VelSlideSystem;
use crate::wire::ZoomSystem;

///
/// Editor Bundle, use full systems for a in-game editor.
///

#[derive(Default)]
pub struct WireEditorBundle;

impl WireEditorBundle {
    /// Create a new transform bundle
    pub fn new() -> Self {
        WireEditorBundle {}
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for WireEditorBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(MoveSystem, "editor_move_system", &[]);
        builder.add(VelSlideSystem, "editor_vel_slide_system", &[]);
        builder.add(ZoomSystem, "editor_zoom_system", &[]);
        Ok(())
    }
}
