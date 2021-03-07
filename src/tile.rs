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
}

pub struct Air {
    out_of_bounds: bool,
}
impl Air {
    pub fn new() -> Self {
        Air {
            out_of_bounds: false,
        }
    }

    pub fn new_out_of_bounds() -> Self {
        Air {
            out_of_bounds: true,
        }
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
        !self.out_of_bounds
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
}

pub struct Gravel;
impl Gravel {
    pub fn new() -> Self {
        Gravel {}
    }
}

impl Tile for Gravel {
    fn render(&self, pos: (usize, usize), _size: (usize, usize), buffer: &mut render::FrameBuffer) {
        buffer.render(pos, &[Style::Fg(100, 100, 100)], '@');
    }
}
