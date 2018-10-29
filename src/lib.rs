#[macro_use]
mod macros;

pub mod ability;
pub mod card;
pub mod mana;
pub mod phase;
pub mod type_;

use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    pub fn iterator() -> impl Iterator<Item = Color> {
        use self::Color::*;

        const COLORS: [Color; 5] = [
            White,
            Blue,
            Black,
            Red,
            Green,
        ];

        COLORS.iter().cloned()
    }

    pub fn color_pie_order(color1: Color, color2: Color) -> (Color, Color) {
        let c = (color1 as u8, color2 as u8);
        if c.0 > c.1 {
            Color::color_pie_order(color2, color1)
        } else if c.1 - c.0 > 2 {
                (color2, color1)
        } else {
            (color1, color2)
        }
    }

    pub fn initial(self) -> char {
        use self::Color::*;

        match self {
            White => 'W',
            Blue => 'U',
            Black => 'B',
            Red => 'R',
            Green => 'G',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct ColorIdentity(u8);

impl ColorIdentity {
    pub fn new() -> ColorIdentity {
        Default::default()
    }

    pub fn is_colorless(self) -> bool {
        self.0.trailing_zeros() >= 5
    }

    pub fn has(self, color: Color) -> bool {
        self.0 & (1 << color as u8) != 0
    }

    pub fn add(&mut self, color: Color) {
        self.0 |= 1 << color as u8;
    }

    pub fn remove(&mut self, color: Color) {
        self.0 &= !(1 << color as u8);
    }

    pub fn colors(self) -> impl Iterator<Item = Color> {
        Color::iterator().filter(move |c| self.has(*c))
    }
}

impl FromIterator<Color> for ColorIdentity {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> ColorIdentity {
        let mut res = ColorIdentity::new();
        for color in iter.into_iter() {
            res.add(color);
        }
        res
    }
}

impl fmt::Display for ColorIdentity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_colorless() {
            write!(f, "C")?;
        } else {
            for color in self.colors() {
                write!(f, "{}", color.initial())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_enum_sanity() {
        use self::Color::*;

        assert_eq!(White as u8, 0);
        assert_eq!(Blue as u8, 1);
        assert_eq!(Black as u8, 2);
        assert_eq!(Red as u8, 3);
        assert_eq!(Green as u8, 4);
    }

    #[test]
    fn test_color_identity() {
        assert_eq!(ColorIdentity(0b00000).to_string(), "C");
        assert_eq!(ColorIdentity(0b10001).to_string(), "WG");
        assert_eq!(ColorIdentity(0b01110).to_string(), "UBR");
        assert_eq!(ColorIdentity(0b01000).to_string(), "R");
    }

    #[test]
    fn color_pie_order() {
        use self::Color::*;

        let valid_pairs = [
            (White, Blue),
            (White, Black),
            (Blue, Black),
            (Blue, Red),
            (Black, Red),
            (Black, Green),
            (Red, Green),
            (Red, White),
            (Green, White),
            (Green, Blue),
        ];

        for c1 in Color::iterator() {
            for c2 in Color::iterator() {
                if c1 == c2 {
                    continue;
                }
                let pair = Color::color_pie_order(c1, c2);
                assert!(valid_pairs.iter().any(|&vp| vp == pair));
            }
        }
    }
}
