use std::io;
use std::io::{stdin, stdout, StdoutLock, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

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

fn render(world: &world::World, out: &mut RawTerminal<StdoutLock<'_>>) -> Result<(), io::Error> {
    let mut framebuffer = render::FrameBuffer::new()?;
    framebuffer.render_world(&world);
    framebuffer.display(out)?;
    Ok(())
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    if !termion::is_tty(&stdout) {
        panic!("Error: Must use aftventure on a tty");
    }
    write!(stdout, "{}", cursor::Hide).unwrap();

    let mut world = world::World::new();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for c in stdin.keys() {
            if let Ok(c) = c {
                if let Err(_) = tx.send(c) {
                    break;
                }
            }
        }
    });

    'out: loop {
        while let Ok(c) = rx.try_recv() {
            match c {
                Key::Char('q') => break 'out,
                Key::Up => world.move_player(0, -1, 0),
                Key::Down => world.move_player(0, 1, 0),
                Key::Left => world.move_player(-1, 0, 0),
                Key::Right => world.move_player(1, 0, 0),
                Key::Char('>') => world.move_player(0, 0, -1),
                Key::Char('<') => world.move_player(0, 0, 1),
                _ => {}
            }
        }

        render(&world, &mut stdout).unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    write!(stdout, "{}{}", clear::All, cursor::Show).unwrap();
}
