use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    event::read,
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

mod shared;
use shared::handlers::*;
use shared::prints::*;
use shared::types::*;

mod windows;
use windows::win::*;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut cur_mode = Mode::Fetch;

    let mut help_toggled: bool = false;
    let mut help_win = Window::new((10, 10), (50, 50), WindowStyle::DoubleStrict);

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout, MoveTo(0, 0))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    print_fetch(&mut stdout)?;

    loop {
        stdout.flush()?;
        if let Some(act) = handle_event(&cur_mode, read()?)? {
            match act {
                Action::Quit => break,
                Action::ToggleHelp => {
                    help_toggled = !help_toggled;
                    if help_toggled {
                        help_win.render(&mut stdout)?;
                    } else {
                        help_win.hide(&mut stdout)?;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
