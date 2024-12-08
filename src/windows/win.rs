use std::{io::Stdout, str::Chars};

use crossterm::{cursor::MoveTo, execute, style::Stylize};

pub struct Window {
    // Upper left corner
    pub x1: u16,
    pub y1: u16,
    // Lower right corner
    pub x2: u16,
    pub y2: u16,

    pub style: WindowStyle,

    overriden: Option<Vec<String>>,
}

impl Window {
    pub fn new(ulc: (u16, u16), lrc: (u16, u16), style: WindowStyle) -> Self {
        Window {
            x1: ulc.0,
            y1: ulc.1,
            x2: lrc.0,
            y2: lrc.1,
            style,
            overriden: None,
        }
    }

    // FIXME: whatever this is
    pub fn render(&mut self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let x1 = self.x1;
        let x2 = self.x2;
        let y1 = self.y1;
        let y2 = self.y2;
        let box_c = self.style.get_chars();

        execute!(stdout, MoveTo(self.x1, self.y1))?;
        print!("{}", box_c[2].bold().red());
        for _ in x1..x2 - 1 {
            print!("{}", box_c[0].bold().red());
        }
        print!("{}", box_c[3].bold().red());

        execute!(stdout, MoveTo(self.x1, self.y1 + 1))?;
        for y in y1 + 1..=y2 {
            print!("{}", box_c[1].bold().red());
            execute!(stdout, MoveTo(self.x1, y))?;
        }

        execute!(stdout, MoveTo(self.x1, self.y2))?;
        print!("{}", box_c[4].bold().red());
        for _ in x1..x2 {
            print!("{}", box_c[0].bold().red());
        }
        execute!(stdout, MoveTo(self.x2, self.y1 + 1))?;
        for y in y1 + 1..=y2 {
            print!("{}", box_c[1].bold().red());
            execute!(stdout, MoveTo(self.x2, y))?;
        }
        execute!(stdout, MoveTo(self.x2, self.y2))?;
        print!("{}", box_c[5].bold().red());
        Ok(())
    }

    // TODO: hide window
    pub fn hide(&mut self, stdout: &Stdout) -> anyhow::Result<()> {
        Ok(())
    }
}

pub enum WindowStyle {
    Rounded,
    Strict,
    DoubleStrict,
}

// 0 -- horizontal line
// 1 -- vertical line
// 2 -- upper left corner
// 3 -- upper right corner
// 4 -- lower left corner
// 5 -- lower right corner
impl WindowStyle {
    fn get_chars(&self) -> Vec<char> {
        match self {
            WindowStyle::Rounded => "─│╭╮╰╯".to_owned().chars().collect::<Vec<char>>(),
            WindowStyle::Strict => "─│┌┐└┘".to_owned().chars().collect::<Vec<char>>(),
            WindowStyle::DoubleStrict => "═║╔╗╚╝".to_owned().chars().collect::<Vec<char>>(),
        }
    }
}
