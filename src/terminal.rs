use crate::Position;
use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyEvent};
use crossterm::execute;
use crossterm::style::{ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::{self, Write};
pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size()?;
        crossterm::terminal::enable_raw_mode().expect("ERROR: raw mode couldnt be enabled");
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn flush() {
        io::stdout().flush().expect("ERROR: flushing screen failed");
    }
    pub fn clear_screen() {
        execute!(io::stdout(), Clear(ClearType::All)).expect("ERROR: clearing screen failed");
    }
    pub fn read_key() -> Result<KeyEvent, std::io::Error> {
        loop {
            match event::read() {
                Ok(e) => match e {
                    Event::Key(k) => {
                        return Ok(k);
                    }
                    _ => continue,
                },
                Err(e) => {
                    return Err(e);
                }
            };
        }
    }
    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = *x as u16;
        let y = *y as u16;
        execute!(io::stdout(), MoveTo(x, y)).expect("ERROR: setting cursor position failed");
    }
    pub fn clear_current_line() {
        execute!(io::stdout(), Clear(ClearType::CurrentLine)).expect("ERROR: clearing line failed");
    }
    pub fn set_bg_color(color: crossterm::style::Color) {
        execute!(io::stdout(), SetBackgroundColor(color))
            .expect("ERROR: couldnt set background color");
    }
    pub fn reset_color() {
        execute!(io::stdout(), ResetColor).expect("ERROR: couldnt reset color");
    }
    pub fn set_fg_color(color: crossterm::style::Color) {
        execute!(io::stdout(), SetForegroundColor(color))
            .expect("ERROR: couldnt set foireground color");
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("ERROR: raw mode couldnt be disabled");
    }
}
