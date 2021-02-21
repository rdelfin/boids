use crate::components::{BoidData, ObstacleData, Position, Velocity};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Entities, Entity, ReadStorage, System, WriteStorage},
};
use nalgebra::Vector2;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

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

    fn run(&mut self, (boid_datas, _, positions, mut velocities, entities): Self::SystemData) {
        // List of boid position and velocities used for determining new velocities
        let all_boids = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(_, p, v, e)| (e, p.0, v.0))
            .collect::<Vec<_>>();

        (&boid_datas, &positions, &mut velocities, &entities)
            .par_join()
            .for_each(|(boid_data, position, velocity, entity)| {
                let (v_sep, v_align, v_coh, v_noise) = (
                    self.separation(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            entity,
                            position.0,
                            boid_data.separation_radius,
                            &all_boids,
                        ),
                    ),
                    self.alignment(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            entity,
                            position.0,
                            boid_data.alignment_radius,
                            &all_boids,
                        ),
                    ),
                    self.cohesion(
                        boid_data,
                        position.0,
                        velocity.0,
                        &self.neighbour_boids(
                            entity,
                            position.0,
                            boid_data.cohesion_radius,
                            &all_boids,
                        ),
                    ),
                    self.noise(boid_data),
                );

                let weighted_vec = boid_data.separation_weight * v_sep
                    + boid_data.alignment_weight * v_align
                    + boid_data.cohesion_weight * v_coh
                    + boid_data.noise_weight * v_noise;
                if !weighted_vec.x.is_nan()
                    && !weighted_vec.y.is_nan()
                    && weighted_vec.norm() != 0.0
                {
                    velocity.0 += weighted_vec;
                }

                // Cap velocity
                if velocity.0.norm() > boid_data.max_speed {
                    velocity.0 = velocity.0.normalize() * boid_data.max_speed;
                }
            });
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
        entity: Entity,
        position: Vector2<f32>,
        radius: f32,
        all_boids: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vec<(Vector2<f32>, Vector2<f32>)> {
        all_boids
            .iter()
            .filter(|(e, _, _)| entity != *e)
            .filter(|(_, p, _)| (position - p).norm() < radius)
            .map(|(_, p, v)| (*p, *v))
            .collect()
    }

    fn separation(
        &self,
        _boid_data: &BoidData,
        position: Vector2<f32>,
        _velocity: Vector2<f32>,
        neighbours: &Vec<(Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_position = neighbours
            .iter()
            .map(|(_, p)| position - p)
            .fold(Vector2::new(0., 0.), |prev, pos| prev + pos)
            / (neighbours.len() as f32);
        position - avg_position
    }

    fn alignment(
        &self,
        _boid_data: &BoidData,
        _position: Vector2<f32>,
        velocity: Vector2<f32>,
        neighbours: &Vec<(Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_direction = neighbours
            .iter()
            .fold(Vector2::new(0.0, 0.0), |prev, (_, curr)| prev + curr)
            / neighbours.len() as f32;
        avg_direction - velocity
    }

    fn cohesion(
        &self,
        _boid_data: &BoidData,
        position: Vector2<f32>,
        _velocity: Vector2<f32>,
        neighbours: &Vec<(Vector2<f32>, Vector2<f32>)>,
    ) -> Vector2<f32> {
        if neighbours.len() == 0 {
            return Vector2::new(0.0, 0.0);
        }

        let avg_position = neighbours
            .iter()
            .fold(Vector2::new(0.0, 0.0), |prev, (pos, _)| prev + pos)
            / (neighbours.len() as f32);
        avg_position - position
    }
}
