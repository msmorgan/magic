use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

use super::Color;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSymbol {
    Generic(u16),
    Colored(Color),
    Colorless,
    Variable,
    Hybrid(Color, Color),
    MonoHybrid(Color),
    Phyrexian(Color),
    Snow,
}

impl fmt::Display for ManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ManaSymbol::*;

        write!(f, "{{")?;
        match *self {
            Generic(n) => write!(f, "{}", n),
            Colored(c) => write!(f, "{}", c),
            Colorless => write!(f, "C"),
            Variable => write!(f, "X"),
            Hybrid(c1, c2) => {
                let (c1, c2) = Color::color_pie_order(c1, c2);
                write!(f, "{}/{}", c1, c2)
            }
            MonoHybrid(c) => write!(f, "{}/2", c),
            Phyrexian(c) => write!(f, "{}/P", c),
            Snow => write!(f, "S"),
        }?;
        write!(f, "}}")
    }
}

pub trait ConvertedManaCost {
    fn converted_mana_cost(&self) -> usize;
}

impl ConvertedManaCost for ManaSymbol {
    fn converted_mana_cost(&self) -> usize {
        use self::ManaSymbol::*;

        match *self {
            Generic(n) => n as usize,
            Colored(_) => 1,
            Colorless => 1,
            Variable => 0,
            Hybrid(_, _) => 1,
            MonoHybrid(_) => 2,
            Phyrexian(_) => 1,
            Snow => 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ManaCost {
    symbols: Vec<ManaSymbol>,
}

#[derive(Default, PartialEq, Eq)]
struct ManaCostComparator {
    generic: usize,
    variable: usize,
    snow: usize,
    colors: HashMap<Option<Color>, usize>,
    hybrids: HashMap<(Color, Option<Color>), usize>,
    phyrexian: HashMap<Color, usize>,
}

impl ManaCostComparator {
    fn new(mana_cost: &ManaCost) -> Self {
        let mut res = ManaCostComparator {
            ..Default::default()
        };

        for symbol in mana_cost.symbols.iter() {
            use self::ManaSymbol::*;

            match *symbol {
                Generic(n) => res.generic += n as usize,
                Colored(c) => *res.colors.entry(Some(c)).or_insert(0) += 1,
                Colorless => *res.colors.entry(None).or_insert(0) += 1,
                Variable => res.variable += 1,
                Hybrid(c1, c2) => {
                    let (c1, c2) = Color::color_pie_order(c1, c2);
                    *res.hybrids.entry((c1, Some(c2))).or_insert(0) += 1;
                }
                MonoHybrid(c) => *res.hybrids.entry((c, None)).or_insert(0) += 1,
                Phyrexian(c) => *res.phyrexian.entry(c).or_insert(0) += 1,
                Snow => res.snow += 1,
            }
        }

        res
    }
}

impl PartialEq for ManaCost {
    fn eq(&self, other: &Self) -> bool {
        ManaCostComparator::new(self) == ManaCostComparator::new(other)
    }
}

impl Eq for ManaCost {}

impl FromIterator<ManaSymbol> for ManaCost {
    fn from_iter<T>(iter: T) -> ManaCost
    where
        T: IntoIterator<Item = ManaSymbol>,
    {
        ManaCost {
            symbols: Vec::from_iter(iter),
        }
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.symbols.iter().try_for_each(|s| write!(f, "{}", s))
    }
}

impl ConvertedManaCost for ManaCost {
    fn converted_mana_cost(&self) -> usize {
        self.symbols.iter().map(|s| s.converted_mana_cost()).sum()
    }
}

#[cfg(test)]
mod tests {
    use self::Color::*;
    use self::ManaSymbol::*;
    use super::*;

    #[test]
    fn mana_symbol_to_string() {
        assert_eq!(Phyrexian(Blue).to_string(), "{U/P}");
        assert_eq!(Generic(0).to_string(), "{0}");
    }

    #[test]
    fn mana_cost_to_string() {
        let cost = ManaCost::from_iter(vec![
            ManaSymbol::Generic(5),
            ManaSymbol::Colorless,
            ManaSymbol::Colored(Green),
            ManaSymbol::Hybrid(Black, White),
        ]);

        assert_eq!(cost.to_string(), "{5}{C}{G}{W/B}");
    }
}
