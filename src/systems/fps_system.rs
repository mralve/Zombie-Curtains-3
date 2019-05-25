use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, ReadExpect, System, WriteStorage, ReadStorage, Entity},
    ui::UiText,
};

pub struct FPSSystem;

pub struct Text {
    pub fps: Entity,
}

impl<'s> System<'s> for FPSSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        Read<'s, Time>,
        ReadExpect<'s, Text>,
    );

    fn run(&mut self, (mut ui_text, time, text_debug): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(text_debug.fps) {
            text.text = format!("FPS: {}", 1.0 / time.delta_seconds());
        }
    }
}