use crate::{Document, Row, Terminal};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::Rng;
use std::{
    env,
    time::{Duration, Instant},
};

const STATUS_FG_COLOR: crossterm::style::Color = crossterm::style::Color::Black;
const STATUS_BG_COLOR: crossterm::style::Color = crossterm::style::Color::Rgb {
    r: 240,
    g: 160,
    b: 200,
};
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

const NUM_STATUS_MESSAGES: usize = 30;
const STATUS_MESSAGES: [&str; NUM_STATUS_MESSAGES] = [
    "ChatGPT: 'Rust-ing Your Editor Skills!'",
    "ChatGPT: 'Code With Rustic Elegance.'",
    "ChatGPT: 'Ctrl+S: Save, Not Sorcery.'",
    "ChatGPT: 'Rustling Up Code Magic.'",
    "ChatGPT: 'Cargo Run Your Creativity!'",
    "ChatGPT: 'Syntax High Five!'",
    "ChatGPT: 'Vim: Meet Your Rusty Rival.'",
    "ChatGPT: 'Rustic Code: Editor's Choice.'",
    "ChatGPT: 'Coding with Rusty Grace.'",
    "ChatGPT: 'rustc: Your BFF in Coding.'",
    "ChatGPT: 'Cargo: Your Code's Best Mate.'",
    "ChatGPT: 'Rust in Peace, Bugs!'",
    "ChatGPT: 'Ctrl+C, Ctrl+V, Ctrl+Rust.'",
    "ChatGPT: 'Rust for the Code Curious.'",
    "ChatGPT: 'Rust, Refactor, Repeat.'",
    "ChatGPT: 'Rustling Up Code Dreams.'",
    "ChatGPT: 'Vim or Rust? Tough Choice!'",
    "ChatGPT: 'Rust: Where Bugs Rust Away.'",
    "ChatGPT: 'Crafting Code with Rust.'",
    "ChatGPT: 'Rust Editor: Less Rust, More Speed.'",
    "ChatGPT: 'Rustic Text: Elegance Defined.'",
    "ChatGPT: 'Ctrl+Z, Ctrl+Rust, Ctrl+Zen.'",
    "ChatGPT: 'Coding, Rusting, & Rejoicing.'",
    "ChatGPT: 'Rust On, Editor Extraordinaire!'",
    "ChatGPT: 'Rustling Code Perfection.'",
    "ChatGPT: 'Rustic Charm for Coders.'",
    "ChatGPT: 'Cargo Run: Magic Unleashed.'",
    "ChatGPT: 'Rust, Refine, Revolutionize.'",
    "ChatGPT: 'Rust: Code's Best Friend.'",
    "ChatGPT: 'Elegance Meets Efficiency.'",
];

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
}
impl Editor {
    #[must_use]
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("ESCAPE to quit | Ctrl-S to save");

        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default();
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("failed to init terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            if let Err(e) = self.process_keypress() {
                die(e);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key: KeyEvent = Terminal::read_key()?;
        if pressed_key.kind == KeyEventKind::Press {
            if pressed_key.modifiers == KeyModifiers::CONTROL {
                match pressed_key.code {
                    KeyCode::Char('s') => {
                        self.save();
                    }
                    _ => (),
                }

                return Ok(());
            }

            match pressed_key.code {
                KeyCode::Esc => {
                    if self.quit_times < 3 && self.document.is_dirty() {
                        self.status_message = StatusMessage::from(format!(
                            "WARNING! Unsaved changes in file! Press ESC {} more times to quit",
                            3 - self.quit_times
                        ));
                        self.quit_times += 1;
                        return Ok(());
                    }
                    self.should_quit = true;
                }
                KeyCode::Char(c) => {
                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(KeyCode::Right);
                }
                KeyCode::Delete => {
                    self.document.delete(&self.cursor_position);
                }
                KeyCode::Backspace => {
                    self.move_cursor(KeyCode::Left);
                    self.document.delete(&self.cursor_position);
                }
                KeyCode::Enter => {
                    self.document.insert(&self.cursor_position, '\n');
                    self.move_cursor(KeyCode::Down);
                }
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => {
                    self.move_cursor(pressed_key.code);
                }
                _ => {}
            };
        }
        self.scroll();

        if self.quit_times > QUIT_TIMES {
            self.quit_times = 0;
            self.status_message = StatusMessage::from(String::new());
        }

        return Ok(());
    }

    fn move_cursor(&mut self, code: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match code {
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            KeyCode::End => x = width,
            KeyCode::Home => x = 0,
            KeyCode::PageUp => {
                y = std::cmp::max(y.saturating_sub(terminal_height), 0);
            }
            KeyCode::PageDown => {
                y = std::cmp::min(y.saturating_add(terminal_height), height);
            }
            _ => (),
        };

        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y };
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = offset.y.saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn refresh_screen(&mut self) {
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
            return;
        }
        Terminal::flush();
        self.draw_rows();
        self.draw_status_bar();
        self.draw_message_bar();
        Terminal::cursor_position(&Position {
            x: self.cursor_position.x.saturating_sub(self.offset.x),
            y: self.cursor_position.y.saturating_sub(self.offset.y),
        });
    }

    fn draw_rows(&self) {
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(self.offset.y + terminal_row as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = start + self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_welcome_message(&self) {
        let welcome_message = format!("GEditor editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        let mut formatted_message = format!("~{}{}", spaces, welcome_message);
        formatted_message.truncate(width);
        println!("{}\r", &formatted_message)
    }

    fn draw_status_bar(&self) {
        let mut status: String;
        let width = self.terminal.size().width as usize;

        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_color();
    }

    fn draw_message_bar(&mut self) {
        Terminal::clear_current_line();
        let time_since_message = Instant::now() - self.status_message.time;
        if time_since_message > Duration::new(8, 0) {
            self.generate_random_message();
        }

        let message = &self.status_message;

        if time_since_message < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn generate_random_message(&mut self) {
        let index = rand::thread_rng().gen_range(0..NUM_STATUS_MESSAGES - 1);
        self.status_message = StatusMessage::from(STATUS_MESSAGES[index].to_string());
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen();
            let e = Terminal::read_key()?;
            if e.kind == KeyEventKind::Press {
                match e.code {
                    KeyCode::Backspace => {
                        if !result.is_empty() {
                            result.pop();
                        }
                    }
                    KeyCode::Esc => {
                        result.truncate(0);
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        self.status_message = StatusMessage::from(String::new());
                        if result.is_empty() {
                            return Ok(None);
                        }
                        return Ok(Some(result));
                    }
                    KeyCode::Char(c) => {
                        if e.modifiers != KeyModifiers::CONTROL {
                            result.push(c);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name: Option<String> = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        self.status_message = if self.document.save().is_ok() {
            StatusMessage::from("File saved succesfully".to_string())
        } else {
            StatusMessage::from("error saving file".to_string())
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{e}");
}
