extern crate piston_window;
extern crate sprite;
extern crate ai_behavior;
extern crate tiled;
use std::path::Path;
use piston_window::*;

fn main() {
    let map = tiled::parse_file(&Path::new("assets/maps/map-2.tmx")).unwrap();

    let (width, height) = (600, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: tiled", [width, height])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let tileset = map.get_tileset_by_gid(1).unwrap();
    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;

    let ref mut texture_context = window.create_texture_context();
    let tileset_src = String::from("assets/maps/") + &tileset.images[0].source;
    println!("tileset: {}", tileset_src);
    
    let tilesheet = tileset_src;
    let tilesheet = Texture::from_path(
        texture_context,
        &tilesheet,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let (width, _) = tilesheet.get_size();
    let layer: &tiled::Layer = &map.layers[0];
    let image = Image::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5; 4], g);

            for (y, row) in layer.tiles.iter().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    if tile == 0 {
                        continue;
                    }

                    let tile = tile - 1; // tiled counts from 1

                    // rect of the particular tile in the tilesheet
                    let src_rect = [
                        (tile % (width / tile_width) * tile_width) as f64,
                        (tile / (width / tile_height) * tile_height) as f64,
                        tile_width as f64,
                        tile_height as f64,
                    ];

                    let trans = c.transform.trans(
                        x as f64 * tile_width as f64,
                        y as f64 * tile_height as f64,
                    );

                    image.src_rect(src_rect).draw(
                        &tilesheet,
                        &DrawState::default(),
                        trans,
                        g,
                    );
                }
            }
        });
    }
}

