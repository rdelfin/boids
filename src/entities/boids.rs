use crate::{
    components::{BoidData, Position, Velocity},
    resources::{SpriteCache, SpriteKey},
};
use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, LazyUpdate, Read},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::{anyhow, Result};
use nalgebra::Vector2;

pub fn fill_boid<'s>(
    entities: &Entities<'s>,
    sprite_cache: &Read<SpriteCache>,
    updater: &LazyUpdate,
    start_pos: Vector2<f32>,
    max_vel: f32,
) -> Result<Entity> {
    let boid_handle = sprite_cache.fetch(SpriteKey::Boid)?.clone();

    Ok(updater
        .create_entity(entities)
        .with(SpriteRender {
            sprite_sheet: boid_handle,
            sprite_number: 0,
        })
        .with(Position(start_pos))
        .with(Velocity(Vector2::new(0., 0.)))
        .with(Transform::default())
        .with(Transparent)
        .with(BoidData {
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            noise_weight: 0.1,
            view_radius: 150.,
            fov_angle: 15. / 8. * std::f32::consts::PI,
            max_speed: max_vel,
        })
        .build())
}
