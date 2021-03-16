extern crate cgmath;

#[macro_use]
extern crate glium;

use cgmath::{vec3, Point3, Vector3};
use glium::{
    glutin::{
        event::{DeviceEvent, Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
    Display,
};

pub mod game;
pub mod generation;
pub mod gfx;
pub mod render;
pub mod tile;
pub mod world;

pub struct Player {
    pub sprite: char,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub position: Point3<f32>,
    pub orientation: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Player {
    pub fn new(x: i32, y: i32, z: i32, sprite: char) -> Self {
        Player {
            sprite,
            x,
            y,
            z,
            position: Point3::new(0.0, 0.0, -1.0),
            orientation: vec3(0.0, 0.0, 1.0),
            up: vec3(0.0, 1.0, 0.0),
        }
    }
}

pub fn run(disp: Display, el: EventLoop<()>) {
    let mut game = game::Game::new(disp);
    let mut now = std::time::Instant::now();

    el.run(move |event, _, control_flow| {
        let new_now = std::time::Instant::now();
        let elapsed = new_now - now;
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    game.input_mouse_motion(delta, elapsed);
                }
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => game.input(input, elapsed),
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::MainEventsCleared => {
                game.update(new_now - now);
                game.render();
                now = new_now;
                std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
            }
            _ => (),
        }
    });
}
