use crate::resources::{SpriteCache, SpriteKey};
use amethyst::{
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::{anyhow, Result};

pub fn new_boid(world: &mut World) -> Result<Entity> {
    let boid_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))
            .unwrap();

        sprite_cache.fetch(SpriteKey::Boid).unwrap().clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: boid_handle,
            sprite_number: 0,
        })
        .with(Transform::default())
        .with(Transparent)
        .build())
}
