use crate::components::{Position, Velocity};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Read, ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Position>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut positions, time): Self::SystemData) {
        for (velocity, position) in (&velocities, &mut positions).join() {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            position.0 += velocity.0 * frame_delta_s;
        }
    }
}

#[derive(SystemDesc)]
pub struct PositionSystem;

impl<'s> System<'s> for PositionSystem {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, Transform>);

    fn run(&mut self, (positions, mut transforms): Self::SystemData) {
        for (position, transform) in (&positions, &mut transforms).join() {
            transform.set_translation_xyz(
                position.0.x.round(),
                position.0.y.round(),
                transform.translation().z,
            );
        }
    }
}
