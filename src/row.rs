use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start: usize = std::cmp::min(start, end);

        let mut result = String::new();
        let mut index = 0;
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            index += 1;
            let spaces_to_tab = (start + index) % 8;

            if grapheme == "\t" {
                result.push_str(&" ".repeat(spaces_to_tab));
            } else {
                result.push_str(grapheme);
            }
        }

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
        };
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}
