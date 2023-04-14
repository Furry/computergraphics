extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::f64::consts::PI;

const CW: f64 = 800.0;
const CH: f64 = 600.0;
const VW: f64 = 4.0; // Adjusted viewport width
const VH: f64 = 3.0; // Adjusted viewport height
const D: f64 = 3.0; // Adjusted distance from the camera

// Add these functions for rotation matrices
fn calc_rotation_x(angle: f64) -> [[f64; 3]; 3] {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    [
        [1.0, 0.0, 0.0],
        [0.0, cos_angle, -sin_angle],
        [0.0, sin_angle, cos_angle],
    ]
}

fn calc_rotation_y(angle: f64) -> [[f64; 3]; 3] {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    [
        [cos_angle, 0.0, sin_angle],
        [0.0, 1.0, 0.0],
        [-sin_angle, 0.0, cos_angle],
    ]
}

fn calc_rotation_z(angle: f64) -> [[f64; 3]; 3] {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    [
        [cos_angle, -sin_angle, 0.0],
        [sin_angle, cos_angle, 0.0],
        [0.0, 0.0, 1.0],
    ]
}

fn viewport_to_canvas(x: f64, y: f64, dx: f64, dy: f64) -> (f64, f64) {
    (x * CW / VW + CW / 2.0 + dx, y * CH / VH + CH / 2.0 + dy) // Center the cube on the canvas and apply the offset
}

fn project_vertex(v: (f64, f64, f64), dx: f64, dy: f64, d: f64) -> Point {
    let (x, y) = viewport_to_canvas(v.0 * d / v.2, v.1 * d / v.2, dx, dy);
    Point::new(x as i32, y as i32)
}

fn draw_triangle(
    canvas: &mut WindowCanvas,
    v1: (f64, f64, f64),
    v2: (f64, f64, f64),
    v3: (f64, f64, f64),
    color: Color,
    dx: f64,
    dy: f64,
    d: f64,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.draw_line(project_vertex(v1, dx, dy, d), project_vertex(v2, dx, dy, d))?;
    canvas.draw_line(project_vertex(v2, dx, dy, d), project_vertex(v3, dx, dy, d))?;
    canvas.draw_line(project_vertex(v3, dx, dy, d), project_vertex(v1, dx, dy, d))?;
    Ok(())
}

fn draw_cube(
    canvas: &mut WindowCanvas,
    dx: f64,
    dy: f64,
    d: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
) -> Result<(), String> {
    let vertices = [
        (-2.0, -0.5, 5.0),
        (-2.0, 0.5, 5.0),
        (-1.0, 0.5, 5.0),
        (-1.0, -0.5, 5.0),
        (-2.0, -0.5, 6.0),
        (-2.0, 0.5, 6.0),
        (-1.0, 0.5, 6.0),
        (-1.0, -0.5, 6.0),
    ];

    let rx = calc_rotation_x(rotation_x);
    let ry = calc_rotation_y(rotation_y);
    let rz = calc_rotation_z(rotation_z);

    let rotated_vertices: Vec<(f64, f64, f64)> = vertices
        .iter()
        .map(|&vertex| {
            let mut rotated = (vertex.0, vertex.1, vertex.2);

            // Apply rotation matrices
            rotated = (
                rx[0][0] * rotated.0 + rx[0][1] * rotated.1 + rx[0][2] * rotated.2,
                rx[1][0] * rotated.0 + rx[1][1] * rotated.1 + rx[1][2] * rotated.2,
                rx[2][0] * rotated.0 + rx[2][1] * rotated.1 + rx[2][2] * rotated.2,
            );
            rotated = (
                ry[0][0] * rotated.0 + ry[0][1] * rotated.1 + ry[0][2] * rotated.2,
                ry[1][0] * rotated.0 + ry[1][1] * rotated.1 + ry[1][2] * rotated.2,
                ry[2][0] * rotated.0 + ry[2][1] * rotated.1 + ry[2][2] * rotated.2,
            );
            rotated = (
                rz[0][0] * rotated.0 + rz[0][1] * rotated.1 + rz[0][2] * rotated.2,
                rz[1][0] * rotated.0 + rz[1][1] * rotated.1 + rz[1][2] * rotated.2,
                rz[2][0] * rotated.0 + rz[2][1] * rotated.1 + rz[2][2] * rotated.2,
            );
            rotated
        }).collect();

    // Define the triangles
    let triangles = [
        (rotated_vertices[0], rotated_vertices[1], rotated_vertices[2], Color::RGB(0, 0, 255)),
        (rotated_vertices[0], rotated_vertices[2], rotated_vertices[3], Color::RGB(0, 0, 255)),
        (rotated_vertices[4], rotated_vertices[5], rotated_vertices[6], Color::RGB(255, 0, 0)),
        (rotated_vertices[4], rotated_vertices[6], rotated_vertices[7], Color::RGB(255, 0, 0)),
        (rotated_vertices[0], rotated_vertices[1], rotated_vertices[5], Color::RGB(0, 255, 0)),
        (rotated_vertices[0], rotated_vertices[5], rotated_vertices[4], Color::RGB(0, 255, 0)),
        (rotated_vertices[1], rotated_vertices[2], rotated_vertices[6], Color::RGB(0, 0, 255)),
        (rotated_vertices[1], rotated_vertices[6], rotated_vertices[5], Color::RGB(0, 0, 255)),
        (rotated_vertices[2], rotated_vertices[3], rotated_vertices[7], Color::RGB(255, 0, 0)),
        (rotated_vertices[2], rotated_vertices[7], rotated_vertices[6], Color::RGB(255, 0, 0)),
        (rotated_vertices[3], rotated_vertices[0], rotated_vertices[4], Color::RGB(0, 255, 0)),
        (rotated_vertices[3], rotated_vertices[4], rotated_vertices[7], Color::RGB(0, 255, 0)),
    ];
    
    for triangle in &triangles {
        draw_triangle(canvas, triangle.0, triangle.1, triangle.2, triangle.3, dx, dy, d)?;
    }
        
    Ok(())
}



fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("project :D", CW as u32, CH as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let mut d: f64 = 4.0;
    let mut rx: f64 = 0.0;
    let mut ry: f64 = 0.0;
    let mut rz: f64 = 0.0;

    let hallway_length: i32 = 10;
    let distance_between_cubes: f64 = 1.0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw_cube(&mut canvas, dx, dy, d, rx, ry, rz)?;

        for i in 0..hallway_length {
            let z_offset = i as f64 * distance_between_cubes;
            draw_cube(
                &mut canvas,
                dx,
                dy,
                d + z_offset,
                rx,
                ry,
                rz // Apply z_offset to the z-axis rotation
            )?;
        }
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
                Keycode::Q => rz += 0.1,
                Keycode::E => rz -= 0.1,
                Keycode::W => rx += 0.1,
                Keycode::S => rx -= 0.1,
                Keycode::A => ry += 0.1,
                Keycode::D => ry -= 0.1,
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

