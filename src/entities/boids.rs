use crate::{
    components::{Position, Velocity},
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

pub fn new_boid(world: &mut World) -> Result<Entity> {
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
        .with(Position(Vector2::new(0., 0.)))
        .with(Velocity(Vector2::new(5., 0.)))
        .with(Transform::default())
        .with(Transparent)
        .build())
}
