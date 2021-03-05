use super::tile;
use super::Player;

use chrono::Utc;
use noise::{NoiseFn, OpenSimplex, Seedable};
use std::collections::HashMap;
use std::io;
use std::io::{StdoutLock, Write};
use std::rc::Rc;
use termion::cursor;
use termion::raw::RawTerminal;

pub struct Chunk {
    x: i32,
    y: i32,
    tiles: Vec<Rc<dyn tile::Tile>>,
    air: Rc<dyn tile::Tile>,
}

impl Chunk {
    pub fn new((x, y): (i32, i32), seed: u32, air: &Rc<dyn tile::Tile>) -> Chunk {
        let mut chunk = Chunk {
            x,
            y,
            tiles: Vec::with_capacity(32 * 32 * 256),
            air: air.clone(),
        };
        let simplex = OpenSimplex::new().set_seed(seed);

        for _ in 0..=32 * 32 * 256 {
            chunk.tiles.push(Rc::new(tile::Air::new()));
        }

        for i in 0..32 {
            for j in 0..32 {
                let scale = 0.08;
                let scaled_x = scale * (i + 32 * x) as f64;
                let scaled_y = scale * (j + 32 * y) as f64;
                let mut val = simplex.get([scaled_x, scaled_y]);
                val += 0.5 * simplex.get([scaled_x / 2.0, scaled_y / 2.0]);
                val += 0.25 * simplex.get([scaled_x / 4.0, scaled_y / 4.0]);
                val += 0.125 * simplex.get([scaled_x / 8.0, scaled_y / 8.0]);
                let stackheight = ((val + 1.0) / 2.0) * 0.75 * 256.0;
                let stackheight = stackheight as i32;

                for k in 0..256 {
                    let tile: Rc<dyn tile::Tile> = if k == stackheight {
                        Rc::new(tile::Grass::new())
                    } else if k > stackheight {
                        Rc::new(tile::Air::new())
                    } else if k < stackheight && k > stackheight - 4 {
                        Rc::new(tile::Dirt::new())
                    } else {
                        Rc::new(tile::Stone::new())
                    };
                    chunk.tiles[(k * 32 * 32 + i * 32 + j) as usize] = tile;
                }
            }
        }

        chunk
    }

    // x, y, z in chunk coords
    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> &Rc<dyn tile::Tile> {
        if x < 0 || x >= 32 {
            &self.air
        } else if y < 0 || y >= 32 {
            &self.air
        } else if z < 0 || z >= 256 {
            &self.air
        } else {
            &self.tiles[(z * 32 * 32 + x * 32 + y) as usize]
        }
    }
}

pub struct World {
    seed: u32,
    air: Rc<dyn tile::Tile>,
    pub player: Player,
    chunk_render_distance: u16,
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            seed: Utc::now().timestamp() as u32,
            air: Rc::new(tile::Air::new_out_of_bounds()),
            player: Player::new(0, 0, 128, '☺'),
            chunk_render_distance: 8,
            chunks: HashMap::new(),
        };

        let half = world.chunk_render_distance / 2;
        let half = half as i32;
        for i in -half..=half {
            for j in -half..=half {
                world.load_chunk((i, j));
            }
        }

        world
    }

    pub fn load_chunk(&mut self, (x, y): (i32, i32)) {
        if self.chunks.contains_key(&(x, y)) {
            return;
        }

        self.chunks
            .insert((x, y), Chunk::new((x, y), self.seed, &self.air));
    }

    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> &Rc<dyn tile::Tile> {
        let chunk_x = x / 32;
        let chunk_y = y / 32;

        if !self.chunks.contains_key(&(chunk_x, chunk_y)) {
            &self.air
        } else {
            self.chunks
                .get(&(chunk_x, chunk_y))
                .unwrap()
                .get(((x % 32).abs(), (y % 32).abs(), z))
        }
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

    pub fn move_player(&mut self, x: i32, y: i32, z: i32) {
        self.player.x += x;
        self.player.y += y;
        self.player.z = (self.player.z + z).max(0).min(255);

        let chunk_x = self.player.x / 32;
        let chunk_y = self.player.y / 32;

        let mut deload: Vec<(i32, i32)> = vec![];
        for (&pos, _) in &self.chunks {
            let dist = (chunk_x - pos.0).abs().max((chunk_y - pos.1).abs());
            if dist > self.chunk_render_distance as i32 {
                deload.push(pos);
            }
        }

        for pos in deload {
            self.chunks.remove(&pos);
        }

        let half = self.chunk_render_distance / 2;
        let half = half as i32;
        for i in -half..=half {
            for j in -half..=half {
                self.load_chunk((i + chunk_x, j + chunk_y));
            }
        }
    }
}
