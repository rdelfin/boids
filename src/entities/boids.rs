use crate::{
    components::{BoidData, Position, Velocity},
    resources::{SpriteCache, SpriteKey},
};
use amethyst::{
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::{anyhow, Result};
use nalgebra::Vector2;

pub fn new_boid(
    world: &mut World,
    start_pos: Vector2<f32>,
    start_vel: Vector2<f32>,
) -> Result<Entity> {
    let boid_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))?;

        sprite_cache.fetch(SpriteKey::Boid)?.clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: boid_handle,
            sprite_number: 0,
        })
        .with(Position(start_pos))
        .with(Velocity(start_vel))
        .with(Transform::default())
        .with(Transparent)
        .with(BoidData {
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            view_radius: 64.,
            fov_angle: 15. / 8. * std::f32::consts::PI,
            speed: start_vel.norm(),
        })
        .build())
}
