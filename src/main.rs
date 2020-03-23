extern crate piston_window;
extern crate sprite;
extern crate ai_behavior;
extern crate tiled;
use std::path::Path;
use piston_window::*;
use piston_window::Button::Keyboard;

fn main() {
    let map = tiled::parse_file(&Path::new("assets/maps/map-2.tmx")).unwrap();

    // player pos and view scale
    let scale = 1.0;
    let (mut x_view, mut y_view) = (0.0, 0.0);

    // window settings
    let (width, height) = (900, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: tiled", [width, height])
        .exit_on_esc(true)
        .resizable(true)
        .controllers(true)
	    .samples(0)
        .graphics_api(opengl)
        .build()
        .unwrap();
 
    let tileset = map.get_tileset_by_gid(1).unwrap();

    let tile_width = tileset.tile_width;
    let tile_height = tileset.tile_height;
    // println!("tilesetmap: {:?}", map);

    let ref mut texture_context = window.create_texture_context();
    let tileset_src = String::from("assets/maps/") + &tileset.images[0].source;
    // println!("tileset: {}", tileset_src);
    
    let tilesheet = tileset_src;
    let tilesheet = Texture::from_path(
        texture_context,
        &tilesheet,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let layer: &tiled::Layer = &map.layers[0];

    // load scene once
    let mut level_src: Vec<[f64;4]> = Vec::new();
    let mut level_dest: Vec<[f64;4]> = Vec::new();
    
    // calculate view dimensions (needed for only loading an drawing the needed tiles)
    // TODO(Cas): fix dit
    // drawing gaat alleen goed als de scale of translation hele getallen zijn
    // let x_min = (x_view - (width as f64/2 as f64) - tile_width as f64) as i32;
    // let x_max = (x_view + (width as f64/2 as f64) + tile_width as f64) as i32;
    // let y_min = (y_view - (height as f64/2 as f64) - tile_height as f64) as i32;
    // let y_max = (y_view + (height as f64/2 as f64) + tile_height as f64) as i32;
    // let x_step = (tile_width as f64/scale) as usize;
    // let y_step = (tile_height as f64/scale) as usize;

    // for (y_tile, y_world) in (y_min..y_max).step_by(y_step).enumerate(){
    //     for (x_tile, x_world) in (x_min..x_max).step_by(x_step).enumerate() {
    //         let tile = get_tile(&layer, x_world, y_world, tile_width, tile_height);

    for (y, row) in layer.tiles.iter().enumerate().clone() {
        for (x, &tile) in row.iter().enumerate() {
            
            if tile == 0 {
                continue;
            }
            
            let (width, _) = tilesheet.get_size();
            let tile = tile - 1; // tiled counts from 1


            // create mapping of source blocks. level_src will be mapped onto level_dest
            // describe tile source block
            level_src.push([
                (tile % (width / tile_width) * tile_width) as f64,
                (tile / (width / tile_height) * tile_height) as f64,
                tile_width as f64,
                tile_height as f64,
            ]);
            
            // describe destination of tile in pixels
            level_dest.push([
                (x as u32* tile_width) as f64,// - x_view as f64,
                (y as u32* tile_height) as f64,// - y_view as f64,
                tile_width as f64,
                tile_height as f64
            ]);
        }
    }


    // draw/event loop
    while let Some(e) = window.next() {
        if let Some(Keyboard(key)) = e.press_args() {
            // deltas have to be whole numbers, otherwise the drawing will have glitches
            match key{
                Key::D => x_view += 5.0,
                Key::A => x_view -= 5.0,
                Key::W => y_view -= 5.0, // up is negative directiion
                Key::S => y_view += 5.0,
                _ => {}
            }
        }


        let bg_color = map.background_colour.unwrap();
        let red: f32 = bg_color.red as f32 / 255.0;
        let green: f32 = bg_color.green as f32 / 255.0;
        let blue: f32 = bg_color.blue as f32 / 255.0;

        window.draw_2d(&e, |c, g, _| {
            if let Some(bg_color) = map.background_colour {
                let (red, green, blue) = (bg_color.red as f32 / 255.0,  bg_color.green as f32 / 255.0,  bg_color.red as f32 / 255.0);
                clear([red, green, blue, 0.5], g);
            } else {
                clear([1.0, 1.0, 1.0, 0.5], g);
            }

            // scale and translate scene
            let trans = c.transform.scale(
                scale as f64,
                scale as f64,
            ).trans(
                -x_view,
                -y_view
            );
            // give normal color for level. Otherwise the level will be tinted
            let color = [1.0; 4];
            
            // draw each tile
            for (&src,&dest) in level_src.iter().zip(level_dest.iter()){
                g.tri_list_uv(
                    &DrawState::default(),
                    &color,
                    &tilesheet,
                    |f| {
                        f(&triangulation::rect_tri_list_xy(trans, dest),
                        &triangulation::rect_tri_list_uv(&tilesheet, src))
                    }
                );
            }
        });
    }
}
