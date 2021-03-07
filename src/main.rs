use std::io;
use std::io::{stdin, stdout, StdoutLock, Write};
use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
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
    let stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let mut world = world::World::new();

    render(&world, &mut stdout).unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => world.move_player(0, -1, 0),
            Key::Down => world.move_player(0, 1, 0),
            Key::Left => world.move_player(-1, 0, 0),
            Key::Right => world.move_player(1, 0, 0),
            Key::Char('>') => world.move_player(0, 0, -1),
            Key::Char('<') => world.move_player(0, 0, 1),
            _ => {}
        }
        render(&world, &mut stdout).unwrap();
    }
    write!(stdout, "{}{}", clear::All, cursor::Show).unwrap();
}
