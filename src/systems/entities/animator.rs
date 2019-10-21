use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter, RonFormat},
    core::transform::{Transform, TransformBundle},
    core::{math::Vector3, timing::Time},
    derive::PrefabData,
    ecs::{prelude::Entity, Component, Entities, Join, ReadStorage, VecStorage, WriteStorage},
    error::Error,
    prelude::{Builder, World, WorldExt},
    renderer::{
        camera::Camera,
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
    Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData, Trans,
};

/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone)]
pub enum AnimationId {
    Idel,
    Walking,
    Running,
    Attack1,
    Attack2,
    Death,
}

#[derive(Default)]
pub struct Animator {
    pub last_x: f32,
    pub distance_trigger: f32,
}

impl Component for Animator {
    type Storage = VecStorage<Self>;
}
