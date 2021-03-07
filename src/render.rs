use std::io;
use std::io::{StdoutLock, Write};
use termion::{color, cursor, raw::RawTerminal, style};

use crate::world::World;
use crate::Player;

#[derive(Clone, Copy)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Blink,
    Fg(u8, u8, u8),
    Bg(u8, u8, u8),
}

fn style_string(style: Style) -> String {
    match style {
        Style::Bold => format!("{}", style::Bold),
        Style::Italic => format!("{}", style::Italic),
        Style::Underline => format!("{}", style::Underline),
        Style::Blink => format!("{}", style::Blink),
        Style::Fg(r, g, b) => format!("{}", color::Fg(color::Rgb(r, g, b))),
        Style::Bg(r, g, b) => format!("{}", color::Bg(color::Rgb(r, g, b))),
    }
}

// (0,0) based not (1,1) based
pub struct FrameBuffer {
    width: usize,
    height: usize,
    chars: Vec<char>,
    styles: Vec<String>,
}

impl FrameBuffer {
    pub fn new() -> Result<Self, io::Error> {
        let (width, height) = termion::terminal_size()?;

        Ok(FrameBuffer {
            width: width as usize,
            height: height as usize,
            chars: vec![' '; (width * height) as usize],
            styles: vec!["".to_string(); (width * height) as usize],
        })
    }

    // Take in screen coords, (1,1) based
    pub fn render(&mut self, (x, y): (usize, usize), style: &[Style], disp: char) {
        let x = x - 1;
        let y = y - 1;

        if x >= self.width {
            return;
        }
        if y >= self.height {
            return;
        }
        self.chars[y * self.width + x] = if disp.is_control() { ' ' } else { disp };

        self.styles[y * self.width + x] = "".to_string();
        for &sty in style {
            self.styles[y * self.width + x] =
                format!("{}{}", self.styles[y * self.width + x], style_string(sty));
        }
    }

    pub fn render_world(&mut self, world: &World) {
        world.render((self.width, self.height), self);
        self.render_player(&world.player);
    }

    pub fn render_player(&mut self, player: &Player) {
        self.render(
            (self.width / 2, self.height / 2),
            &[Style::Bold],
            player.sprite,
        );
    }

    pub fn display(&mut self, out: &mut RawTerminal<StdoutLock<'_>>) -> Result<(), io::Error> {
        let mut display = format!("{}", cursor::Goto(1, 1));

        for y in 0..self.height {
            for x in 0..self.width {
                display = format!(
                    "{}{}{}{}",
                    display,
                    style::Reset,
                    self.styles[y * self.width + x],
                    self.chars[y * self.width + x],
                );
            }
        }
        writeln!(out, "{}", display)?;

        Ok(())
    }
}
