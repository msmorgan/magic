use super::Color;

use std::iter::FromIterator;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum ManaSymbol {
    Generic(usize),
    Color(Option<Color>),
    Variable,
    Hybrid(Color, Color),
    MonoHybrid(Color),
    Phyrexian(Color),
    Snow,
}

impl ManaSymbol {
    pub fn generic(amount: usize) -> ManaSymbol {
        ManaSymbol::Generic(amount)
    }

    pub fn colorless() -> ManaSymbol {
        ManaSymbol::Color(None)
    }

    pub fn colored(color: Color) -> ManaSymbol {
        ManaSymbol::Color(Some(color))
    }

    pub fn variable() -> ManaSymbol {
        ManaSymbol::Variable
    }

    pub fn hybrid(color1: Color, color2: Color) -> ManaSymbol {
        let (color1, color2) = Color::color_pie_order(color1, color2);
        ManaSymbol::Hybrid(color1, color2)
    }

    pub fn mono_hybrid(color: Color) -> ManaSymbol {
        ManaSymbol::MonoHybrid(color)
    }

    pub fn snow() -> ManaSymbol {
        ManaSymbol::Snow
    }

    pub fn phyrexian(color: Color) -> ManaSymbol {
        ManaSymbol::Phyrexian(color)
    }
}

impl fmt::Display for ManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ManaSymbol::*;

        write!(f, "{{{}}}", match *self {
            Generic(n) => n.to_string(),
            Color(None) => "C".to_string(),
            Color(Some(c)) => c.initial().to_string(),
            Variable => "X".to_string(),
            Hybrid(c1, c2) => format!("{}/{}", c1.initial(), c2.initial()),
            MonoHybrid(c) => format!("{}/2", c.initial()),
            Phyrexian(c) => format!("{}/P", c.initial()),
            Snow => "S".to_string(),
        })
    }
}

pub trait ConvertedManaCost {
    fn converted_mana_cost(&self) -> usize;
}

impl ConvertedManaCost for ManaSymbol {
    fn converted_mana_cost(&self) -> usize {
        use self::ManaSymbol::*;

        match *self {
            Generic(n) => n,
            Color(_) => 1,
            Variable => 0,
            Hybrid(_, _) => 1,
            MonoHybrid(_) => 2,
            Phyrexian(_) => 1,
            Snow => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManaCost {
    symbols: Vec<ManaSymbol>,
}

impl FromIterator<ManaSymbol> for ManaCost {
    fn from_iter<T>(iter: T)  -> ManaCost
        where T: IntoIterator<Item = ManaSymbol> {
        ManaCost { symbols: Vec::from_iter(iter) }
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&String::from_iter(self.symbols.iter().map(ToString::to_string)))
    }
}

impl ConvertedManaCost for ManaCost {
    fn converted_mana_cost(&self) -> usize {
        self.symbols.iter().map(|s| s.converted_mana_cost()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::Color::*;

    #[test]
    fn mana_symbol_to_string() {
        assert_eq!(ManaSymbol::phyrexian(Blue).to_string(), "{U/P}");
        assert_eq!(ManaSymbol::generic(0).to_string(), "{0}");
    }

    #[test]
    fn mana_cost_to_string() {
        let cost = ManaCost::from_iter(vec![
            ManaSymbol::generic(5),
            ManaSymbol::colorless(),
            ManaSymbol::colored(Green),
            ManaSymbol::hybrid(Black, White),
        ]);

        assert_eq!(cost.to_string(), "{5}{C}{G}{W/B}");
    }
}
