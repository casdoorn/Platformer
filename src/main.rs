use amethyst::prelude::*;
use std::env;

mod state;
mod bundle_loader;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = env::current_dir()?;

    // define asset and config paths
    let resources = app_root.join("resources");
    let configs = resources.join("config");
    let display_config = configs.join("display_config.ron");
    let key_bindings = configs.join("keybindings.ron");

    // bundle all game systems
    let game_data = GameDataBuilder::default();
    let game_data = bundle_loader::load_bundles(game_data, key_bindings, display_config);

    let mut game = Application::new(resources, state::MyState, game_data)
                .expect("ERROR: Application creation failed\n");
    game.run();

    Ok(())
}
