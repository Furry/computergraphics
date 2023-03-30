extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

const CW: f64 = 800.0;
const CH: f64 = 600.0;
const VW: f64 = 4.0; // Adjusted viewport width
const VH: f64 = 3.0; // Adjusted viewport height
const D: f64 = 3.0; // Adjusted distance from the camera

fn viewport_to_canvas(x: f64, y: f64, dx: f64, dy: f64) -> (f64, f64) {
    (x * CW / VW + CW / 2.0 + dx, y * CH / VH + CH / 2.0 + dy) // Center the cube on the canvas and apply the offset
}

fn project_vertex(v: (f64, f64, f64), dx: f64, dy: f64, d: f64) -> Point {
    let (x, y) = viewport_to_canvas(v.0 * d / v.2, v.1 * d / v.2, dx, dy);
    Point::new(x as i32, y as i32)
}

fn draw_cube(canvas: &mut WindowCanvas, dx: f64, dy: f64, d: f64) -> Result<(), String> {
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
    canvas.draw_line(project_vertex(v_af, dx, dy, d), project_vertex(v_bf, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_bf, dx, dy, d), project_vertex(v_cf, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_cf, dx, dy, d), project_vertex(v_df, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_df, dx, dy, d), project_vertex(v_af, dx, dy, d))?;

    canvas.set_draw_color(red);
    canvas.draw_line(project_vertex(v_ab, dx, dy, d), project_vertex(v_bb, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_bb, dx, dy, d), project_vertex(v_cb, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_cb, dx, dy, d), project_vertex(v_db, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_db, dx, dy, d), project_vertex(v_ab, dx, dy, d))?;

    canvas.set_draw_color(green);
    canvas.draw_line(project_vertex(v_af, dx, dy, d), project_vertex(v_ab, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_bf, dx, dy, d), project_vertex(v_bb, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_cf, dx, dy, d), project_vertex(v_cb, dx, dy, d))?;
    canvas.draw_line(project_vertex(v_df, dx, dy, d), project_vertex(v_db, dx, dy, d))?;

    Ok(())
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
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let mut d: f64 = 4.0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_cube(&mut canvas, dx, dy, d)?;

        canvas.present();

        for event in sdl_context.event_pump()?.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
            Event::KeyDown {
                keycode: Some(keycode),
                keymod,
                ..
            } => match keycode {
                Keycode::Escape => break 'running,
                Keycode::Up => dy -= 10.0,
                Keycode::Down => dy += 10.0,
                Keycode::Left => dx -= 10.0,
                Keycode::Right => dx += 10.0,
                Keycode::Space => d *= 0.9, // Zoom in by decreasing D
                _ => {
                    if keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD)
                        || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)
                    {
                        d *= 1.1; // Zoom out by increasing D
                    }
                }
            },
            _ => {}
        }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

