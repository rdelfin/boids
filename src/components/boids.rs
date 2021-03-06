use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct BoidData {
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub noise_weight: f32,
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,
    pub max_speed: f32,
}

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct ObstacleData {
    pub separation_weight: f32,
    pub separation_radius: f32,
}
