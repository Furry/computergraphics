extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

const CW: f64 = 800.0;
const CH: f64 = 600.0;
const VW: f64 = 4.0; 
const VH: f64 = 3.0; 
const D: f64 = 3.0; 

fn viewport_to_canvas(x: f64, y: f64) -> (f64, f64) {
    (x * CW / VW + CW / 2.0, y * CH / VH + CH / 2.0)
}

fn project_vertex(v: (f64, f64, f64)) -> Point {
    let (x, y) = viewport_to_canvas(v.0 * D / v.2, v.1 * D / v.2);
    Point::new(x as i32, y as i32)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("3D Perspective Projection", CW as u32, CH as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let v_af = (-2.0, -0.5, 5.0);
    let v_bf = (-2.0, 0.5, 5.0);
    let v_cf = (-1.0, 0.5, 5.0);
    let v_df = (-1.0, -0.5, 5.0);

    let v_ab = (-2.0, -0.5, 6.0);
    let v_bb = (-2.0, 0.5, 6.0);
    let v_cb = (-1.0, 0.5, 6.0);
    let v_db = (-1.0, -0.5, 6.0);

    let blue = Color::RGB(0, 0, 255);
    let red = Color::RGB(255, 0, 0);
    let green = Color::RGB(0, 255, 0);

    canvas.set_draw_color(blue);
    canvas.draw_line(project_vertex(v_af), project_vertex(v_bf))?;
    canvas.draw_line(project_vertex(v_bf), project_vertex(v_cf))?;
    canvas.draw_line(project_vertex(v_cf), project_vertex(v_df))?;
    canvas.draw_line(project_vertex(v_df), project_vertex(v_af))?;

    canvas.set_draw_color(red);
    canvas.draw_line(project_vertex(v_ab), project_vertex(v_bb))?;
    canvas.draw_line(project_vertex(v_bb), project_vertex(v_cb))?;
    canvas.draw_line(project_vertex(v_cb), project_vertex(v_db))?;
    canvas.draw_line(project_vertex(v_db), project_vertex(v_ab))?;

    canvas.set_draw_color(green);
    canvas.draw_line(project_vertex(v_af), project_vertex(v_ab))?;
    canvas.draw_line(project_vertex(v_bf), project_vertex(v_bb))?;
    canvas.draw_line(project_vertex(v_cf), project_vertex(v_cb))?;
    canvas.draw_line(project_vertex(v_df), project_vertex(v_db))?;

    canvas.present();

    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit { .. }
                | Event::KeyDown {

                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
