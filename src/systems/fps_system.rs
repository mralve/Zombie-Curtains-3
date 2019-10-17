use amethyst::{
    core::timing::Time,
    ecs::prelude::{
        Component, Entity, Join, NullStorage, Read, ReadExpect, ReadStorage, System, WriteStorage,
    },
    ui::UiText,
};

#[derive(Default)]
pub struct FPSComp;

impl FPSComp {
    pub fn new() -> FPSComp {
        FPSComp {}
    }
}

impl Component for FPSComp {
    type Storage = NullStorage<Self>;
}

pub struct FPSSystem;

pub struct Text {
    pub fps: Entity,
}

impl<'s> System<'s> for FPSSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        Read<'s, FPSComp>,
        Read<'s, Time>,
        ReadExpect<'s, Text>,
    );

    fn run(&mut self, (mut ui_text, fcomp, time, text_debug): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(text_debug.fps) {
            text.text = format!("FPS: {}", 1.0 / time.delta_seconds());
        }
    }
}
