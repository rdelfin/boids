use crate::components::{BoidData, Velocity};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct BoidSystem;

impl<'s> System<'s> for BoidSystem {
    type SystemData = (ReadStorage<'s, BoidData>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (boid_datas, mut velocities): Self::SystemData) {
        for (boid_data, velocity) in (&boid_datas, &mut velocities).join() {
            velocity.0.x = boid_data.speed;
        }
    }
}
