use amethyst::{
    input::{InputBundle, StringBindings},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle}
};
use std::env;

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = env::current_dir()?;

    // define asset and config paths
    let resources = app_root.join("resources");
    let configs = resources.join("config");
    let display_config = configs.join("display_config.ron");
    let key_bindings = configs.join("keybindings.ron");

    // bundle all game systems
    let game_data = GameDataBuilder::default()
        // Transform bundle
        .with_bundle(TransformBundle::new())?
        // Input handler
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(
                key_bindings.clone(),
            ).expect(
                &format!("ERROR: key binding file loading failed, file path: {:?}\n", key_bindings)
            ),
        ).expect("ERROR: Registering InputBundle failed\n")
        // UI Interface
        .with_bundle(UiBundle::<StringBindings>::new()).expect("ERROR: Registering UIbundle failed\n")
        // 2D Renderer
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config.clone()).expect(
                            &format!("ERROR: RenderBundle display_config loading failed, file path: {:?}\n", display_config)
                        )
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        ).expect("ERROR: Registering RenderBundle failed\n");

    let mut game = Application::new(resources, state::MyState, game_data)
                .expect("ERROR: Application creation failed\n");
    game.run();

    Ok(())
}
