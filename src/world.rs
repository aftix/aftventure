use crate::render;
use crate::tile;
use crate::tile::Tile;
use crate::Player;

use chrono::Utc;
use noise::{NoiseFn, OpenSimplex, Seedable};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Chunk {
    x: i32,
    y: i32,
    tiles: Vec<usize>,
    air_id: usize,
}

impl Chunk {
    pub fn new((x, y): (i32, i32), seed: u32, map: &HashMap<String, usize>) -> Chunk {
        let air = tile::Air::new();
        let air_id = *map.get(&air.name()).unwrap();
        let mut chunk = Chunk {
            x,
            y,
            tiles: vec![air_id; 32 * 32 * 256],
            air_id,
        };
        let simplex = OpenSimplex::new().set_seed(seed);

        let dirt = tile::Dirt::new();
        let stone = tile::Stone::new();
        let grass = tile::Grass::new();

        let dirt = *map.get(&dirt.name()).unwrap();
        let stone = *map.get(&stone.name()).unwrap();
        let grass = *map.get(&grass.name()).unwrap();

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
                    let tile = if k == stackheight {
                        grass
                    } else if k > stackheight {
                        air_id
                    } else if k < stackheight && k > stackheight - 4 {
                        dirt
                    } else {
                        stone
                    };
                    chunk.tiles[(k * 32 * 32 + i * 32 + j) as usize] = tile;
                }
            }
        }

        chunk
    }

    // x, y, z in chunk coords
    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> usize {
        if x < 0 || x >= 32 {
            self.air_id
        } else if y < 0 || y >= 32 {
            self.air_id
        } else if z < 0 || z >= 256 {
            self.air_id
        } else {
            self.tiles[(z * 32 * 32 + x * 32 + y) as usize]
        }
    }
}

pub struct World {
    seed: u32,
    pub player: Player,
    chunk_render_distance: u16,
    chunks: HashMap<(i32, i32), Chunk>,
    tile_map: HashMap<String, usize>,
    tiles: Vec<Box<dyn Tile>>,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            seed: Utc::now().timestamp() as u32,
            player: Player::new(0, 0, 128, '☺'),
            chunk_render_distance: 8,
            chunks: HashMap::new(),
            tile_map: HashMap::new(),
            tiles: vec![],
        };

        tile::add_tiles(&mut world.tiles);

        for (i, tile) in world.tiles.iter().enumerate() {
            world.tile_map.insert(tile.name(), i as usize);
        }

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
            .insert((x, y), Chunk::new((x, y), self.seed, &self.tile_map));
    }

    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> &Box<dyn Tile> {
        let chunk_x = x / 32;
        let chunk_y = y / 32;

        let air = tile::Air::new();
        let air_id = *self.tile_map.get(&air.name()).unwrap();

        if !self.chunks.contains_key(&(chunk_x, chunk_y)) {
            &self.tiles[air_id]
        } else {
            &self.tiles[self.chunks.get(&(chunk_x, chunk_y)).unwrap().get((
                (x % 32).abs(),
                (y % 32).abs(),
                z,
            ))]
        }
    }

    pub fn render(&self, (width, height): (usize, usize), buffer: &mut render::FrameBuffer) {
        let screen_x = width / 2;
        let screen_y = height / 2;

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
                        .render_top((i, j), (width, height), buffer);
                } else {
                    tile.render((i, j), (width, height), buffer);
                }
            }
        }
        for i in 1..=width {
            buffer.render((i, 1), &[], '#');
        }
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
