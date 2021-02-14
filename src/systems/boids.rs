use crate::components::{BoidData, Position, Velocity};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Entities, Entity, ReadStorage, System, WriteStorage},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct BoidSystem;

impl<'s> System<'s> for BoidSystem {
    type SystemData = (
        ReadStorage<'s, BoidData>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, Velocity>,
        Entities<'s>,
    );

    fn run(&mut self, (boid_datas, positions, mut velocities, entities): Self::SystemData) {
        // List of boid position and velocities used for determining new velocities
        let all_boids = (&boid_datas, &positions, &velocities, &entities)
            .join()
            .map(|(_, p, v, e)| (e, p.0, v.0))
            .collect::<Vec<_>>();

        for (boid_data, position, velocity, entity) in
            (&boid_datas, &positions, &mut velocities, &entities).join()
        {
            let neighbour_boids =
                self.neighbour_boids(entity, position.0, velocity.0, &boid_data, &all_boids);

            let (v_sep, v_align, v_coh) = (
                self.separation(boid_data, position.0, velocity.0, &neighbour_boids),
                self.alignment(boid_data, position.0, velocity.0, &neighbour_boids),
                self.cohesion(boid_data, position.0, velocity.0, &neighbour_boids),
            );

            let weighted_vec = boid_data.separation_weight * v_sep
                + boid_data.alignment_weight * v_align
                + boid_data.cohesion_weight * v_coh;
            if !weighted_vec.x.is_nan() && !weighted_vec.y.is_nan() && weighted_vec.norm() != 0.0 {
                velocity.0 += weighted_vec.normalize() * boid_data.speed;
            }
        }
    }
}

impl BoidSystem {
    fn neighbour_boids(
        &self,
        entity: Entity,
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        boid_data: &BoidData,
        all_boids: &Vec<(Entity, Vector2<f32>, Vector2<f32>)>,
    ) -> Vec<(Vector2<f32>, Vector2<f32>)> {
        all_boids
            .iter()
            .filter(|(e, _, _)| entity != *e)
            .filter(|(_, p, _)| (position - p).norm() < boid_data.view_radius)
            .filter(|(_, _, v)| v.angle(&velocity).abs() < boid_data.fov_angle / 2.0)
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

        neighbours
            .iter()
            .map(|(_, p)| position - p)
            .fold(Vector2::new(0., 0.), |prev, pos| prev + pos)
            .normalize()
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
            .map(|(_, v)| v.normalize())
            .fold(Vector2::new(0.0, 0.0), |prev, curr| prev + curr)
            .normalize();
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
