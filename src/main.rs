mod snake_state;
mod systems;

use crate::snake_state::Snake;
use std::time::Duration;
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow, RenderDebugLines},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    core::frame_limiter::FrameRateLimitStrategy,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::DirectionSystem, "direction_system", &["input_system"])
        .with(systems::MoveSystem::default(), "move_system", &[])
        .with(systems::FoodSystem, "food_system", &[]);


    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    // let mut game = Application::build(assets_dir, Snake::default())?
    //                 .with_frame_limit(FrameRateLimitStrategy::Yield, 10)
    //                 .build(game_data)?;
    let mut game = Application::new(assets_dir, Snake::default(), game_data)?;
   
    game.run();

    Ok(())
}
