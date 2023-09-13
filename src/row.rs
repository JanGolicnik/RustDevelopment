use crate::{highlighting, HighlightingOptions};
use crossterm::style::Stylize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
    pub is_highlighted: bool,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
            highlighting: Vec::new(),
            is_highlighted: false,
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
        self.is_highlighted = false;

        return Self {
            string: splitted_row,
            len: splitted_length,
            highlighting: Vec::new(),
            is_highlighted: false,
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

    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }

            let mut index = 0;

            while let Some(search_match) = self.find(word, index) {
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    for i in index.saturating_add(search_match)..next_index {
                        self.highlighting[i] = highlighting::Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    fn highlight_comment(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if highlighting_options.comments() && c == '/' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                    }
                    return true;
                }
            }
        }
        return false;
    }

    fn highlight_string(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if highlighting_options.strings() && c == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *index += 1;

                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.highlighting.push(highlighting::Type::String);
            *index += 1;
            return true;
        }
        return false;
    }

    fn highlight_number(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if highlighting_options.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                let prev_char = chars[*index - 1];
                if !is_separator(prev_char) {
                    return false;
                }
            }

            loop {
                self.highlighting.push(highlighting::Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '.' || next_char.is_ascii_digit() {
                        continue;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return true;
        }
        return false;
    }

    fn highlight_character(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if highlighting_options.characters() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };
                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlighting.push(highlighting::Type::Character);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        highlighting_type: highlighting::Type,
    ) -> bool {
        if substring.is_empty() {
            return false;
        }

        if *index > 0 {
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }

        for (substring_char_index, substring_char) in substring.chars().enumerate() {
            if let Some(row_char) = chars.get(index.saturating_add(substring_char_index)) {
                if *row_char != substring_char {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..substring.len() {
            self.highlighting.push(highlighting_type);
            *index += 1;
        }

        return true;
    }

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        words: &Vec<String>,
        chars: &[char],
        highlighting_type: highlighting::Type,
    ) -> bool {
        for word in words {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars[*index + word.len()];
                if !is_separator(next_char) {
                    continue;
                }
            }

            if self.highlight_str(index, word, chars, highlighting_type) {
                return true;
            }
        }
        return false;
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        return self.highlight_keywords(
            index,
            highlighting_options.primary_keywords(),
            chars,
            highlighting::Type::PrimaryKeyword,
        );
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        return self.highlight_keywords(
            index,
            highlighting_options.secondary_keywords(),
            chars,
            highlighting::Type::SecondaryKeyword,
        );
    }

    fn highlight_multiline_comments(
        &mut self,
        index: &mut usize,
        highlighting_options: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if highlighting_options.multiline_comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(highlighting::Type::MultilineComment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        return false;
    }

    pub fn highlight(
        &mut self,
        highlighting_options: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();

        if self.is_highlighted && word.is_none() {
            if let Some(highlighting_type) = self.highlighting.last() {
                if *highlighting_type == highlighting::Type::MultilineComment
                    && self.string.len() > 1
                    && &self.string[self.string.len() - 2..] == "*/"
                {
                    return true;
                }
            }
            return false;
        }

        self.highlighting = Vec::new();
        let mut index = 0;

        let mut in_multiline_comment = start_with_comment;

        if in_multiline_comment {
            let closing_index = if let Some(closing_index) = self.string[index..].find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };

            for _ in 0..closing_index {
                self.highlighting.push(highlighting::Type::MultilineComment);
            }
            index = closing_index;
        }

        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comments(&mut index, highlighting_options, *c, &chars) {
                in_multiline_comment = true;
                continue;
            }

            in_multiline_comment = false;

            if self.highlight_character(&mut index, highlighting_options, *c, &chars)
                || self.highlight_comment(&mut index, highlighting_options, *c, &chars)
                || self.highlight_primary_keywords(&mut index, highlighting_options, &chars)
                || self.highlight_secondary_keywords(&mut index, highlighting_options, &chars)
                || self.highlight_string(&mut index, highlighting_options, *c, &chars)
                || self.highlight_number(&mut index, highlighting_options, *c, &chars)
            {
                continue;
            } else {
                self.highlighting.push(highlighting::Type::None);
                index += 1;
            }
        }
        self.highlight_match(word);

        if in_multiline_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }

        self.is_highlighted = true;
        return false;
    }
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}
