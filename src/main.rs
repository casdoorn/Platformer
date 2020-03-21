extern crate piston_window;
extern crate sprite;
extern crate ai_behavior;

use std::rc::Rc;
use uuid;

use piston_window::*;
use sprite::*;

use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

pub struct SpriteInfo{
    x: f64,
    y: f64,
    ID: uuid::Uuid
}

fn main() {
    let (width, height) = (600, 600);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("main window", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut scene = Scene::new();
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let tex = Rc::new(Texture::from_path(
            &mut texture_context,
            assets.join("random.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
    
    // Calculate background positions
    let mut background_pos: Vec<[f64;2]> = Vec::new();
    for x in (32..width).step_by(64){
        for y in (32..height).step_by(64){
            background_pos.push([x as f64, y as f64]);
        }
    }

    // keep track of generated 
    let mut background_ids: Vec<SpriteInfo> = Vec::new();
    // fill background with random.png images
    for sprite_pos in background_pos{
        let mut sprite = Sprite::from_texture(tex.clone());
        sprite.set_position(sprite_pos[0], sprite_pos[1]);
        // add sprite to the scene
        let id = scene.add_child(sprite);
        // store x,y and id of the sprite
        background_ids.push(SpriteInfo{
                                x: sprite_pos[0], 
                                y: sprite_pos[1],
                                ID: id}
                            );
    }

    // Run a sequence of animations.
    // let seq = Sequence(vec![
    //     Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
    //     Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
    //     Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
    //     Wait(0.5),
    //     Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Blink(1.0, 5)),
    //     While(Box::new(WaitForever), vec![
    //         Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
    //         Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
    //     ]),
    // ]);
    // scene.run(id, &seq);

    // // This animation and the one above can run in parallel.
    // let rotate = Action(Ease(EaseFunction::ExponentialInOut,
    //     Box::new(RotateTo(2.0, 360.0))));
    // scene.run(id, &rotate);

    println!("Press any key to pause/resume the animation!");

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });
        if let Some(_) = e.press_args() {
            // scene.toggle(id, &seq);
            // scene.toggle(id, &rotate);
        }
    }
}
