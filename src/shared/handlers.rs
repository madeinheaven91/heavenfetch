use crossterm::event::{Event, KeyCode};

use crate::{Action, Mode};

pub fn handle_event(mode: &Mode, ev: Event) -> anyhow::Result<Option<Action>> {
    if let Event::Key(key) = ev {
        match key.code {
            KeyCode::Char('q') => return Ok(Some(Action::Quit)),
            KeyCode::Char('?') => return Ok(Some(Action::ToggleHelp)),
            KeyCode::Char('f') => return Ok(Some(Action::EnterFetch)),
            KeyCode::Char('S') => return Ok(Some(Action::EnterSystem)),
            KeyCode::Char('u') => return Ok(Some(Action::EnterUser)),
            KeyCode::Char('n') => return Ok(Some(Action::EnterNetworks)),
            KeyCode::Char('s') => return Ok(Some(Action::EnterSession)),
            KeyCode::Char('e') => return Ok(Some(Action::EnterEnv)),
            _ => (),
        }
    };
    match mode {
        Mode::Fetch => handle_fetch(ev),
        Mode::Env => handle_env(ev),
        _ => unimplemented!(),
    }
}

// TODO: have fetch and other mode windows have multiple panes that can be turned on/off
pub fn handle_fetch(ev: Event) -> anyhow::Result<Option<Action>> {
    match ev {
        Event::Key(key) => match key {
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

pub fn handle_env(ev: Event) -> anyhow::Result<Option<Action>> {
    match ev {
        _ => Ok(None),
    }
}
