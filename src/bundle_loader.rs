
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::UiBundle
};

use std::path::*;

pub fn load_bundles<'a, 'b>(game: GameDataBuilder<'a, 'b>, key_bindings: PathBuf, display_config: PathBuf) -> GameDataBuilder<'a, 'b>{
    game.with_bundle(
        TransformBundle::new()
    ).expect("ERROR: Registering TransformBundle failed\n")
    // Input handler
    .with_bundle(
        InputBundle::<StringBindings>::new()
        .with_bindings_from_file(
            key_bindings.clone(),
        ).expect(
            &format!("ERROR: key binding file loading failed, file path: {:?}\n", key_bindings)
        )
    ).expect("ERROR: Registering InputBundle failed\n")
    // UI Interface
    .with_bundle(
        UiBundle::<StringBindings>::new()
    ).expect("ERROR: Registering UIbundle failed\n")
    // 2D Renderer
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config.clone())
            .expect(&format!("ERROR: RenderBundle display_config loading failed, file path: {:?}\n", display_config)
            )
            .with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
    ).expect("ERROR: Registering RenderBundle failed\n")
}