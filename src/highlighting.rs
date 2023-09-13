use crossterm::style::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeyword,
    SecondaryKeyword,
}

impl Type {
    pub fn to_color(self) -> Color {
        match self {
            Type::Number => Color::Rgb {
                r: 150,
                g: 200,
                b: 255,
            },
            Type::Match => Color::Rgb {
                r: 38,
                g: 139,
                b: 210,
            },
            Type::String => Color::Rgb {
                r: 230,
                g: 230,
                b: 60,
            },
            Type::Character => Color::Rgb {
                r: 230,
                g: 200,
                b: 30,
            },
            Type::Comment | Type::MultilineComment => Color::Rgb {
                r: 100,
                g: 200,
                b: 100,
            },
            Type::PrimaryKeyword => Color::Rgb {
                r: 255,
                g: 100,
                b: 180,
            },
            Type::SecondaryKeyword => Color::Rgb {
                r: 180,
                g: 255,
                b: 100,
            },
            _ => Color::White,
        }
    }
}
