use amethyst::ecs::{Component, VecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Vector2<f32>);

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2<f32>);
