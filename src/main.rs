extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use shapes::Drawable;
use std::time::Duration;

pub mod shapes;
pub mod canvas;

use canvas::Canvas;

use shapes::Color;

fn init_sdl(title: &str, width: u32, height: u32) -> (Canvas, EventPump) {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    (
        Canvas {
            canvas: window.into_canvas().accelerated().build().unwrap(),
        },
        context.event_pump().unwrap(),
    )
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let (mut canvas, mut event_pump) = init_sdl("Rust SDL2", WIDTH, HEIGHT);

    let mut i = 0;
    let mut square = shapes::Rectangle::new(100.0, 100.0, shapes::geometry::Point::new(100.0, 100.0));
    let mut triangle = shapes::Triangle::new(
        shapes::geometry::Point::new(10.0, 10.0),
        shapes::geometry::Point::new(100.0, 10.0),
        shapes::geometry::Point::new(10.0, 100.0),
    );

    'running: loop {
        canvas.clear();
        i += 1;
        square.draw(&mut canvas, Color::new(122, 122, 122));
        square.fill(&mut canvas, Color::new(122, 122, 122));

        triangle.draw(&mut canvas, Color::new(122, 122, 122));
        triangle.fill_gradient(&mut canvas, Color::new(122, 122, 122));

        // triangle.draw(&mut canvas, Color::new(122,122,122));
        // square.set_rotation(square.theta + 0.001);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
