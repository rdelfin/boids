use crate::resources::{SpriteCache, SpriteKey};
use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender, Transparent},
    window::ScreenDimensions,
};
use anyhow::{anyhow, Result};
use nalgebra::Vector3;

use log::info;

pub struct MyState;

impl SimpleState for MyState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        load_sprites(world);

        let boid_handle = {
            let sprite_cache = world
                .try_fetch::<SpriteCache>()
                .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while crating player."))
                .unwrap();

            sprite_cache.fetch(SpriteKey::Boid).unwrap().clone()
        };

        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: boid_handle,
                sprite_number: 0,
            })
            .with(Transform::default())
            .with(Transparent)
            .build();
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.);
    transform.set_scale(Vector3::new(0.2, 0.2, 1.));

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) {
    let mut sprite_cache = SpriteCache::new();
    sprite_cache.load(SpriteKey::Boid, world);
    world.insert(sprite_cache);
}
