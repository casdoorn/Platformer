extern crate piston_window;
extern crate sprite;
extern crate ai_behavior;

use piston_window::*;
use piston_window::Button::Keyboard;

fn main() {
    // player pos and view scale
    let scale = 1.0;
    let (mut x_view, mut y_view) = (0.0, 0.0);

    // window settings
    let (width, height) = (900, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston: tiled", [width, height])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

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

        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 0.0, 1.0, 0.5], g);

            // scale and translate scene
            let trans = c.transform.scale(
                scale as f64,
                scale as f64,
            ).trans(
                -x_view,
                -y_view
            );
        });
    }
}
