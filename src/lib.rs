extern crate cgmath;

#[macro_use]
extern crate glium;

use glium::{
    glutin::{
        event::{Event, WindowEvent},
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
}

impl Player {
    pub fn new(x: i32, y: i32, z: i32, sprite: char) -> Self {
        Player { sprite, x, y, z }
    }
}

pub fn run(disp: Display, el: EventLoop<()>) {
    let mut game = game::Game::new(disp);
    let mut now = std::time::Instant::now();

    el.run(move |event, _, control_flow| {
        let new_now = std::time::Instant::now();
        match event {
            Event::WindowEvent { event, .. } => match event {
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
