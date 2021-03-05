use chrono::Utc;
use noise::{NoiseFn, OpenSimplex, Seedable};
use std::io;
use std::io::{stdin, stdout, StdoutLock, Write};
use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
};

mod tile;

struct World {
    tiles: Vec<Box<dyn tile::Tile>>,
    width: i32,
    height: i32,
    depth: i32,
    seed: u32,
    air: Box<dyn tile::Tile>,
    pub player: Player,
}

struct Player {
    pub sprite: char,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl World {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        let mut world = World {
            tiles: vec![],
            width,
            height,
            depth,
            seed: Utc::now().timestamp() as u32,
            air: Box::new(tile::Air::new_out_of_bounds()),
            player: Player::new(width / 2, height / 2, depth - 1, '☺'),
        };

        for _ in 0..width {
            for _ in 0..height {
                for _ in 0..depth {
                    world.tiles.push(Box::new(tile::Air::new()));
                }
            }
        }

        let simplex = OpenSimplex::new().set_seed(world.seed);

        for i in 0..width {
            for j in 0..height {
                let scaled_x = 40.0 * i as f64 / width as f64;
                let scaled_y = 40.0 * j as f64 / height as f64;
                let mut val = simplex.get([scaled_x, scaled_y]);
                val += 0.5 * simplex.get([scaled_x / 2.0, scaled_y / 2.0]);
                val += 0.25 * simplex.get([scaled_x / 4.0, scaled_y / 4.0]);
                val += 0.125 * simplex.get([scaled_x / 8.0, scaled_y / 8.0]);
                let stackheight = ((val + 1.0) / 2.0) * 0.75 * depth as f64;
                let stackheight = stackheight as i32;
                for k in 0..depth {
                    let tile: Box<dyn tile::Tile> = if k == stackheight {
                        Box::new(tile::Grass::new())
                    } else if k > stackheight {
                        Box::new(tile::Air::new())
                    } else if k < stackheight && k > stackheight - 4 {
                        Box::new(tile::Dirt::new())
                    } else {
                        Box::new(tile::Stone::new())
                    };
                    world.tiles[(k * width * depth + i * width + j) as usize] = tile;
                }
            }
        }

        while world
            .get((world.player.x, world.player.y, world.player.z))
            .habitable()
        {
            world.player.z -= 1;
        }
        world.player.z += 1;

        world
    }

    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> &Box<dyn tile::Tile> {
        if x < 0 || x >= self.width {
            return &self.air;
        }
        if y < 0 || y >= self.height {
            return &self.air;
        }
        if z < 0 || z >= self.depth {
            return &self.air;
        }

        &self.tiles[(z * self.depth * self.width + x * self.width + y) as usize]
    }

    pub fn render(
        &self,
        (width, height): (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        let screen_x = width / 2;
        let screen_y = width / 2;

        // Screen space to world space
        let convert = |x: i32, y: i32| {
            let shift_x = x - screen_x as i32;
            let shift_y = y - screen_y as i32;
            (self.player.x + shift_x, self.player.y + shift_y)
        };

        for i in 1..=width {
            for j in 1..=height {
                let coords = convert(i as i32, j as i32);
                let tile = self.get((coords.0, coords.1, self.player.z));
                if tile.transparent() {
                    self.get((coords.0, coords.1, self.player.z - 1))
                        .render_top((i, j), (width, height), out)?;
                } else {
                    tile.render((i, j), (width, height), out)?;
                }
            }
        }

        Ok(())
    }
}

impl Player {
    pub fn new(x: i32, y: i32, z: i32, sprite: char) -> Self {
        Player { sprite, x, y, z }
    }
}

fn render(world: &World, out: &mut RawTerminal<StdoutLock<'_>>) -> Result<(), io::Error> {
    let (width, height) = termion::terminal_size()?;
    let screen_x = width / 2;
    let screen_y = height / 2;

    write!(out, "{}", clear::All)?;
    world.render((width, height), out)?;

    writeln!(
        out,
        "{}{}{}{}",
        cursor::Goto(screen_x, screen_y),
        style::Bold,
        world.player.sprite,
        style::Reset,
    )?;
    Ok(())
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let mut world = World::new(256, 256, 256);

    render(&world, &mut stdout).unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => world.player.y = (world.player.y - 1).max(0),
            Key::Down => world.player.y = (world.player.y + 1).min(world.height - 1),
            Key::Left => world.player.x = (world.player.x - 1).max(0),
            Key::Right => world.player.x = (world.player.x + 1).min(world.width - 1),
            Key::Char('>') => world.player.z = (world.player.z - 1).max(0),
            Key::Char('<') => world.player.z = (world.player.z + 1).min(world.depth - 1),
            _ => {}
        }
        render(&world, &mut stdout).unwrap();
    }
    write!(stdout, "{}{}", clear::All, cursor::Show).unwrap();
}
