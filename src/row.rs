use crate::highlighting;
use crossterm::style::Stylize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
            highlighting: Vec::new(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start: usize = std::cmp::min(start, end);

        let mut current_highlighting = &highlighting::Type::None;

        let mut result = String::new();
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            let spaces_to_tab = (start + index) % 8;

            let highlighting_type = self
                .highlighting
                .get(index)
                .unwrap_or(&highlighting::Type::None);

            if current_highlighting != highlighting_type {
                current_highlighting = highlighting_type;
            }

            if grapheme == "\t" {
                result.push_str(&" ".repeat(spaces_to_tab));
            } else {
                result.push_str(&format!(
                    "{}",
                    grapheme.to_string().with(current_highlighting.to_color())
                ));
            }
        }

        result.push_str(&"".with(crossterm::style::Color::Reset).to_string());

        return result;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }

        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result = String::new();
            let mut len = 0;
            for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
                if index != at {
                    len += 1;
                    result.push_str(grapheme);
                }
            }
            self.string = result;
            self.len = len;
        }
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row = String::new();
        let mut length = 0;
        let mut splitted_row = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;

        return Self {
            string: splitted_row,
            len: splitted_length,
            highlighting: Vec::new(),
        };
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str, from: usize) -> Option<usize> {
        if from > self.len {
            return None;
        }

        let substring: String = self.string[..].graphemes(true).skip(from).collect();

        if let Some(matching_byte_index) = substring.find(query) {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(from + grapheme_index);
                }
            }
        }
        return None;
    }

    pub fn rfind(&self, query: &str, to: usize) -> Option<usize> {
        if to > self.len {
            return None;
        }

        let substring: String = self.string[..].graphemes(true).take(to).collect();

        if let Some(matching_byte_index) = substring.rfind(query) {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(grapheme_index);
                }
            }
        }
        return None;
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        let mut highlighting: Vec<highlighting::Type> = Vec::new();

        let chars: Vec<char> = self.string.chars().collect();

        let mut matches = Vec::new();
        let mut search_index = 0;

        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index) {
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }

        let mut prev_is_separator = true;

        let mut index = 0;
        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word[..].graphemes(true) {
                        index += 1;
                        highlighting.push(highlighting::Type::Match);
                    }
                    continue;
                }
            }

            let previous_highlight = if index > 0 {
                highlighting
                    .get(index - 1)
                    .unwrap_or(&highlighting::Type::None)
            } else {
                &highlighting::Type::None
            };

            if (c.is_ascii_digit()
                && (previous_highlight == &highlighting::Type::Number || prev_is_separator))
                || (c == &'.' && previous_highlight == &highlighting::Type::Number)
            {
                highlighting.push(highlighting::Type::Number);
            } else {
                highlighting.push(highlighting::Type::None);
            }

            prev_is_separator = c.is_ascii_whitespace() || c.is_ascii_punctuation();

            index += 1;
        }

        self.highlighting = highlighting;
    }
}
