use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod entities;
mod input;
mod resources;
mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let config = app_root.join("config");
    let display_config = config.join("display.ron");
    let key_bindings_path = config.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(
            InputBundle::<input::ControlBindingTypes>::new()
                .with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(systems::BoidSystem, "boid_system", &[])
        .with(systems::PhysicsSystem, "physics_system", &["boid_system"])
        .with(
            systems::PositionSystem,
            "position_system",
            &["physics_system"],
        )
        .with(
            systems::MouseInputSystem::default(),
            "mouse_input_system",
            &["position_system"],
        );

    let mut game = Application::new(resources, state::MyState, game_data)?;
    game.run();

    Ok(())
}
