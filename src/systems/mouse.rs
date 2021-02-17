use crate::{
    entities::boids::fill_boid,
    input::{ActionBinding, ControlBindingTypes},
    resources::SpriteCache,
};
use amethyst::{
    core::{geometry::Plane, transform::Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System},
    input::InputHandler,
    renderer::camera::{ActiveCamera, Camera},
    window::ScreenDimensions,
};
use nalgebra::{Point2, Vector2};

#[derive(SystemDesc, Default)]
pub struct MouseInputSystem {
    place_prev_pressed: bool,
}

impl<'s> System<'s> for MouseInputSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<ControlBindingTypes>>,
        Read<'s, SpriteCache>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            lazy_update,
            active_camera,
            input,
            sprite_cache,
            screen_dimensions,
            cameras,
            transforms,
        ): Self::SystemData,
    ) {
        let mouse = match input.mouse_position() {
            Some((x, y)) => Point2::new(x, y),
            None => Point2::new(0.0, 0.0),
        };
        let place_pressed = input.action_is_down(&ActionBinding::Place).unwrap_or(false);
        let mut camera_join = (&cameras, &transforms).join();
        if let Some((camera, camera_transform)) = active_camera
            .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
        {
            // Project a ray from the camera to the 0z axis
            let ray = camera.screen_ray(
                mouse,
                Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                camera_transform,
            );
            let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
            let mouse_pos = ray.at_distance(distance);

            if !place_pressed && self.place_prev_pressed {
                fill_boid(
                    &entities,
                    &sprite_cache,
                    &lazy_update,
                    Vector2::new(mouse_pos.x, mouse_pos.y),
                )
                .unwrap();
            }
        }

        self.place_prev_pressed = place_pressed;
    }
}
