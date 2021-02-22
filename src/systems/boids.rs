use crate::components::{BoidData, ObstacleData, Position, Velocity};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Entities, Entity, ReadStorage, System, WriteStorage},
};
use itertools::izip;
use nalgebra::Vector2;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct BoidSystem;

impl<'s> System<'s> for BoidSystem {
    type SystemData = (
        ReadStorage<'s, BoidData>,
        ReadStorage<'s, ObstacleData>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, Velocity>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (boid_datas, obstacle_datas, positions, mut velocities, entities): Self::SystemData,
    ) {
        // List of boid position and velocities used for determining new velocities
        let all_boids = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(_, p, v, e)| (e, p.0, v.0))
            .collect::<Vec<_>>();

        // Calculate the different vectors
        let separation_map = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(boid_data, position, velocity, entity)| {
                (
                    entity,
                    self.separation(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            Some(entity),
                            position.0,
                            boid_data.separation_radius,
                            &all_boids,
                        ),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let alignment_map = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(boid_data, position, velocity, entity)| {
                (
                    entity,
                    self.alignment(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            Some(entity),
                            position.0,
                            boid_data.alignment_radius,
                            &all_boids,
                        ),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        let cohesion_map = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(boid_data, position, velocity, entity)| {
                (
                    entity,
                    self.cohesion(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            Some(entity),
                            position.0,
                            boid_data.cohesion_radius,
                            &all_boids,
                        ),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let noise_map = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(boid_data, position, velocity, entity)| (entity, self.noise(boid_data)))
            .collect::<Vec<_>>();

        let obstacle_map = self.calculate_obstacles(&all_boids, &obstacle_datas, &positions);

        izip!(
            separation_map,
            alignment_map,
            cohesion_map,
            noise_map,
            obstacle_map
        )
        .for_each(
            |((entity, v_sep), (_, v_align), (_, v_coh), (_, v_noise), (_, v_obstacle))| {
                let mut write_vel = velocities.get_mut(entity).unwrap();
                let boid_data = boid_datas.get(entity).unwrap();

                let weighted_vec = boid_data.separation_weight * v_sep
                    + boid_data.alignment_weight * v_align
                    + boid_data.cohesion_weight * v_coh
                    + boid_data.noise_weight * v_noise
                    // Weight is already incorporated in the obstacle
                    + v_obstacle;
                if !weighted_vec.x.is_nan()
                    && !weighted_vec.y.is_nan()
                    && weighted_vec.norm() != 0.0
                {
                    write_vel.0 += weighted_vec;
                }

                // Cap velocity
                if write_vel.0.norm() > boid_data.max_speed {
                    write_vel.0 = write_vel.0.normalize() * boid_data.max_speed;
                }
            },
        );
    }
}

impl BoidSystem {
    fn noise(&self, boid_data: &BoidData) -> Vector2<f32> {
        let mut rng = thread_rng();
        let angle_dist = Uniform::new(0., 2. * std::f32::consts::PI);
        let speed_dist = Uniform::new(0., 1.);
        let angle = angle_dist.sample(&mut rng);
        let speed = speed_dist.sample(&mut rng);
        boid_data.max_speed * speed * Vector2::new(angle.cos(), angle.sin())
    }

    fn neighbour_boids(
        &self,
        entity: Option<Entity>,
        position: Vector2<f32>,
        radius: f32,
        all_boids: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vec<(Entity, Vector2<f32>, Vector2<f32>)> {
        match entity {
            Some(entity) => all_boids
                .iter()
                .filter(|(e, _, _)| entity != *e)
                .filter(|(_, p, _)| (position - p).norm() < radius)
                .cloned()
                .collect(),
            None => all_boids
                .iter()
                .filter(|(_, p, _)| (position - p).norm() < radius)
                .cloned()
                .collect(),
        }
    }

    fn separation(
        &self,
        _boid_data: &BoidData,
        position: Vector2<f32>,
        _velocity: Vector2<f32>,
        neighbours: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_position = neighbours
            .iter()
            .map(|(_, _, p)| position - p)
            .fold(Vector2::new(0., 0.), |prev, pos| prev + pos)
            / (neighbours.len() as f32);
        position - avg_position
    }

    fn alignment(
        &self,
        _boid_data: &BoidData,
        _position: Vector2<f32>,
        velocity: Vector2<f32>,
        neighbours: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_direction = neighbours
            .iter()
            .fold(Vector2::new(0.0, 0.0), |prev, (_, _, curr)| prev + curr)
            / neighbours.len() as f32;
        avg_direction - velocity
    }

    fn cohesion(
        &self,
        _boid_data: &BoidData,
        position: Vector2<f32>,
        _velocity: Vector2<f32>,
        neighbours: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_position = neighbours
            .iter()
            .fold(Vector2::new(0.0, 0.0), |prev, (_, pos, _)| prev + pos)
            / (neighbours.len() as f32);
        avg_position - position
    }

    fn calculate_obstacles(
        &self,
        all_boids: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
        obstacle_datas: &ReadStorage<ObstacleData>,
        positions: &ReadStorage<Position>,
    ) -> Vec<(Entity, Vector2<f32>)> {
        // Construct returned struct
        let mut avoidance_vecs = all_boids
            .iter()
            .map(|(entity, _, _)| (*entity, Vector2::new(0., 0.)))
            .collect::<Vec<_>>();
        let avoidance_map = all_boids
            .iter()
            .enumerate()
            .map(|(i, (entity, _, _))| (entity, i))
            .collect::<HashMap<_, _>>();

        for (obstacle_data, position) in (obstacle_datas, positions).join() {
            let neighbours =
                self.neighbour_boids(None, position.0, obstacle_data.separation_radius, all_boids);
            for (boid_entity, boid_position, _) in neighbours {
                avoidance_vecs[avoidance_map[&boid_entity]].1 +=
                    (boid_position - position.0) * obstacle_data.separation_weight;
            }
        }

        avoidance_vecs
    }
}
