


pub enum Color {
    Korple,
    Black,
    White,
    Red,
    Lime,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Silver,
    Grey,
    Maroon,
    Olive,
    Green,
    Purple,
    Teal,
    Navy,
    Orange,
    Brown,
    Pink,
    Coral,
    RGB(u8, u8, u8),
    Hex(&'static str)
}

impl Color {
    pub fn rgb(&self, opacity: f64) -> wgpu::Color {
        let (r, g, b) = match self {
            Color::Korple => (1.0, 1.0, 1.0),
            Color::Black => (0.0, 0.0, 0.0),
            Color::White => (1.0, 1.0, 1.0),
            Color::Red => (1.0, 0.0, 0.0),
            Color::Lime => (0.0, 1.0, 0.0),
            Color::Blue => (0.0, 0.0, 1.0),
            Color::Yellow => (1.0, 1.0, 0.0),
            Color::Cyan => (0.0, 1.0, 1.0),
            Color::Magenta => (1.0, 0.0, 1.0),
            Color::Silver => (0.753, 0.753, 0.753),
            Color::Grey => (0.5, 0.5, 0.5),
            Color::Maroon => (0.5, 0.0, 0.0),
            Color::Olive => (0.5, 0.0, 0.0),
            Color::Green => (0.0, 0.5, 0.0),
            Color::Purple => (0.5, 0.0, 0.5),
            Color::Teal => (0.0, 0.5, 0.5),
            Color::Navy => (0.0, 0.0, 0.5),
            Color::Orange => (1.0, 0.65, 0.0),
            Color::Brown => (0.65, 0.165, 0.165),
            Color::Pink => (1.0, 0.753, 0.796),
            Color::Coral => (1.0, 0.5, 0.314),
            Color::RGB(x, y, z) => {
                (x.clone() as f64 / 255.0, y.clone() as f64 / 255.0, z.clone() as f64 / 255.0)
            },
            Color::Hex(s) => {
                match s.len() {
                    7 => {
                        let x = u8::from_str_radix(&s[1..=2], 16).unwrap_or(128) as f64;
                        let y = u8::from_str_radix(&s[3..=4] ,16).unwrap_or(128) as f64;
                        let z = u8::from_str_radix(&s[5..=6], 16).unwrap_or(128) as f64;
                        (x / 255.0, y / 255.0, z / 255.0)
                    },
                    6 => {
                        let x = u8::from_str_radix(&s[0..=1], 16).unwrap_or(128) as f64;
                        let y = u8::from_str_radix(&s[2..=3] ,16).unwrap_or(128) as f64;
                        let z = u8::from_str_radix(&s[4..=5], 16).unwrap_or(128) as f64;
                        (x / 255.0, y / 255.0, z / 255.0)
                    },
                    _ => (0.5, 0.5, 0.5),
                }
            }
        };

        wgpu::Color {
            r, g, b, a: opacity
        }
    }
}