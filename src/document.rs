use crate::FileType;
use crate::Position;
use crate::Row;
use std::fs;
use std::io::Error;
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        return Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type: FileType::from(filename),
        });
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        return self.rows.get(index);
    }

    pub fn is_empty(&self) -> bool {
        return self.rows.is_empty();
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }

        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }

        self.unhilight_rows(at.y);
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();

        if at.y >= len {
            return;
        }

        self.dirty = true;

        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }

        self.unhilight_rows(at.y);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name);
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        self.dirty = false;
        return Ok(());
    }

    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    pub fn find(&self, query: &str, from: &Position) -> Option<Position> {
        if from.y >= self.rows.len() {
            return None;
        }

        let mut position = Position {
            x: from.x,
            y: from.y,
        };

        for _ in from.y..self.rows.len() {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x) {
                    position.x = x;
                    return Some(position);
                }
                position.y = position.y.saturating_add(1);
                position.x = 0;
            } else {
                return None;
            }
        }
        return None;
    }

    pub fn rfind(&self, query: &str, to: &Position) -> Option<Position> {
        if to.y >= self.rows.len() {
            return None;
        }

        let mut position = Position { x: to.x, y: to.y };

        for _ in 0..to.y.saturating_add(1) {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.rfind(&query, position.x) {
                    position.x = x;
                    return Some(position);
                }
                position.y = position.y.saturating_sub(1);
                position.x = self.rows[position.y].len();
            } else {
                return None;
            }
        }
        return None;
    }

    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;

        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };

        for row in &mut self.rows[..until] {
            start_with_comment = row.highlight(
                &self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    fn unhilight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }
}
