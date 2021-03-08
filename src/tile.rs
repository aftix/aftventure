use crate::render;
use crate::render::Style;

pub trait Tile {
    fn render(&self, pos: (usize, usize), size: (usize, usize), buffer: &mut render::FrameBuffer);

    fn render_top(
        &self,
        pos: (usize, usize),
        _size: (usize, usize),
        buffer: &mut render::FrameBuffer,
    ) {
        buffer.render(pos, &[], '.');
    }

    fn habitable(&self) -> bool {
        false
    }

    fn transparent(&self) -> bool {
        false
    }

    fn name(&self) -> String;
}

pub fn add_tiles(vec: &mut Vec<Box<dyn Tile>>) {
    vec.push(Box::new(Air::new()));
    vec.push(Box::new(Dirt::new()));
    vec.push(Box::new(Stone::new()));
    vec.push(Box::new(Grass::new()));
}

pub struct Air;
impl Air {
    pub fn new() -> Self {
        Air {}
    }
}

impl Tile for Air {
    fn render(&self, pos: (usize, usize), _size: (usize, usize), buffer: &mut render::FrameBuffer) {
        buffer.render(pos, &[], ' ');
    }

    fn render_top(
        &self,
        pos: (usize, usize),
        _size: (usize, usize),
        buffer: &mut render::FrameBuffer,
    ) {
        buffer.render(pos, &[], ' ');
    }

    fn habitable(&self) -> bool {
        true
    }

    fn transparent(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        "air".to_string()
    }
}

pub struct Grass;
impl Grass {
    pub fn new() -> Self {
        Grass {}
    }
}

impl Tile for Grass {
    fn render(&self, pos: (usize, usize), _size: (usize, usize), buffer: &mut render::FrameBuffer) {
        buffer.render(pos, &[Style::Fg(150, 75, 0), Style::Bold], '"');
    }

    fn render_top(
        &self,
        pos: (usize, usize),
        _size: (usize, usize),
        buffer: &mut render::FrameBuffer,
    ) {
        buffer.render(pos, &[Style::Fg(0, 255, 0), Style::Bold], '"');
    }

    fn name(&self) -> String {
        "grass".to_string()
    }
}

pub struct Dirt;
impl Dirt {
    pub fn new() -> Self {
        Dirt {}
    }
}

impl Tile for Dirt {
    fn render(&self, pos: (usize, usize), _size: (usize, usize), buffer: &mut render::FrameBuffer) {
        buffer.render(pos, &[Style::Fg(150, 75, 0)], '"');
    }

    fn name(&self) -> String {
        "dirt".to_string()
    }
}

pub struct Stone;
impl Stone {
    pub fn new() -> Self {
        Stone {}
    }
}

impl Tile for Stone {
    fn render(&self, pos: (usize, usize), _size: (usize, usize), buffer: &mut render::FrameBuffer) {
        buffer.render(pos, &[Style::Fg(128, 128, 128)], '#');
    }

    fn name(&self) -> String {
        "stone".to_string()
    }
}
