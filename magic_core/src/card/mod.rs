use super::ColorIdentity;
use crate::mana::{ConvertedManaCost, ManaCost};
use crate::traits::Named;
use std::borrow::Cow;

#[macro_use]
mod type_line;

pub use self::type_line::TypeLine;

// TODO: Add a type that supports X loyalty
type Loyalty = i32;

// TODO: Add a type that supports * for power and/or toughness
type Power = i32;
type Toughness = i32;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CardData {
    name: String,
    mana_cost: ManaCost,
    color_indicator: Option<ColorIdentity>,
    type_line: TypeLine,
    power_toughness: Option<(Power, Toughness)>,
    loyalty: Option<Loyalty>,
}

impl ConvertedManaCost for CardData {
    fn converted_mana_cost(&self) -> usize {
        self.mana_cost.converted_mana_cost()
    }
}

impl Named for CardData {
    fn name(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(&self.name))
    }
}

pub enum Card {
    Normal(CardData),
    Split {
        left: CardData,
        right: CardData,
        fuse: bool,
    },
    Flip {
        top: CardData,
        bottom: CardData,
    },
    DoubleFaced {
        front: CardData,
        back: CardData,
    },
}

impl Named for Card {
    fn name(&self) -> Option<Cow<str>> {
        match self {
            Card::Normal(card_data) => card_data.name(),
            Card::Split { .. } => None,
            Card::Flip { .. } => None,
            Card::DoubleFaced { .. } => None,
        }
    }

    fn names(&self) -> Vec<Cow<str>> {
        match self {
            Card::Normal(card_data) => vec![card_data.name().unwrap()],
            Card::Split { left, right, .. } => vec![left.name().unwrap(), right.name().unwrap()],
            Card::Flip { top, bottom } => vec![top.name().unwrap(), bottom.name().unwrap()],
            Card::DoubleFaced { front, back } => vec![front.name().unwrap(), back.name().unwrap()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_names() {
        let card = Card::Normal(CardData {
            name: "Mountain".to_string(),
            type_line: type_line!(Basic; Land; Mountain),
            ..Default::default()
        });

        assert_eq!(card.name(), Some(Cow::Borrowed("Mountain")));
    }
}
