
use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity}
};

pub use amethyst::core::transform::*;
pub use amethyst::renderer::SpriteRender;

// --------------------------------------------- HEALTH
pub struct Health{
    hp: u32
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}


// --------------------------------------------- PHYSICS
pub struct Physics{
    // TODO: Replace with a better representation
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Component for Physics {
    type Storage = DenseVecStorage<Self>;
}

// --------------------------------------------- ITEM
pub enum Items{
    Health,
    Shield,
}

pub struct Item{
    item: Items,
}

impl Component for Item{
    type Storage = DenseVecStorage<Self>;
}