use std::io;
use std::io::{StdoutLock, Write};
use termion::{color, cursor, raw::RawTerminal, style};

pub trait Tile {
    fn render(
        &self,
        pos: (u16, u16),
        size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error>;

    fn render_top(
        &self,
        pos: (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(out, "{}.", cursor::Goto(pos.0, pos.1))?;
        Ok(())
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
    fn render(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(out, "{}", cursor::Goto(col, row))?;
        Ok(())
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
    fn render(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(
            out,
            "{}{}{}\"{}",
            cursor::Goto(col, row),
            color::Fg(color::Rgb(150, 75, 0)),
            style::Bold,
            style::Reset
        )?;
        Ok(())
    }

    fn render_top(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(
            out,
            "{}{}{}\"{}",
            cursor::Goto(col, row),
            color::Fg(color::Green),
            style::Bold,
            style::Reset
        )?;
        Ok(())
    }
}

pub struct Dirt;
impl Dirt {
    pub fn new() -> Self {
        Dirt {}
    }
}

impl Tile for Dirt {
    fn render(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(
            out,
            "{}{}\"{}",
            cursor::Goto(col, row),
            color::Fg(color::Rgb(150, 75, 0)),
            style::Reset
        )?;
        Ok(())
    }
}

pub struct Stone;
impl Stone {
    pub fn new() -> Self {
        Stone {}
    }
}

impl Tile for Stone {
    fn render(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(
            out,
            "{}{}#{}",
            cursor::Goto(col, row),
            color::Fg(color::Rgb(128, 128, 128)),
            style::Reset
        )?;
        Ok(())
    }
}

pub struct Gravel;
impl Gravel {
    pub fn new() -> Self {
        Gravel {}
    }
}

impl Tile for Gravel {
    fn render(
        &self,
        (col, row): (u16, u16),
        _size: (u16, u16),
        out: &mut RawTerminal<StdoutLock<'_>>,
    ) -> Result<(), io::Error> {
        write!(
            out,
            "{}{}@{}",
            cursor::Goto(col, row),
            color::Fg(color::Rgb(100, 100, 100)),
            style::Reset
        )?;
        Ok(())
    }
}
