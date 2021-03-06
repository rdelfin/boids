use crate::{
    components::{BoidData, ObstacleData, Position, Velocity},
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
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

pub fn fill_boid<'s>(
    entities: &Entities<'s>,
    sprite_cache: &Read<SpriteCache>,
    updater: &LazyUpdate,
    start_pos: Vector2<f32>,
) -> Result<Entity> {
    let boid_handle = sprite_cache.fetch(SpriteKey::Boid)?.clone();
    const MAX_VEL: f32 = 500.;

    Ok(updater
        .create_entity(entities)
        .with(SpriteRender {
            sprite_sheet: boid_handle,
            sprite_number: 0,
        })
        .with(Position(start_pos))
        .with(Velocity(get_boid_vel(MAX_VEL)))
        .with(Transform::default())
        .with(Transparent)
        .with(BoidData {
            separation_weight: 0.1,
            alignment_weight: 0.02,
            cohesion_weight: 1.0,
            noise_weight: 0.1,
            separation_radius: 75.,
            alignment_radius: 150.,
            cohesion_radius: 150.,
            max_speed: MAX_VEL,
        })
        .build())
}

pub fn new_obstacle(world: &mut World, start_pos: Vector2<f32>) -> Result<Entity> {
    let obstacle_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))?;
        sprite_cache.fetch(SpriteKey::Obstacle)?.clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: obstacle_handle,
            sprite_number: 0,
        })
        .with(Position(start_pos))
        .with(Transform::default())
        .with(Transparent)
        .with(ObstacleData {
            separation_weight: 0.2,
            separation_radius: 200.,
        })
        .build())
}

fn get_boid_vel(max_vel: f32) -> Vector2<f32> {
    let mut rng = thread_rng();
    let dir_dist = Uniform::new(0.0, 2. * std::f32::consts::PI);
    let vel_dist = Uniform::new(0.0, max_vel);
    let (dir, vel) = (dir_dist.sample(&mut rng), vel_dist.sample(&mut rng));

    vel * Vector2::new(dir.cos(), dir.sin())
}
