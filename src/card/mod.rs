use mana::ManaCost;
use super::{Color, ColorIdentity};

mod type_line;

pub use self::type_line::TypeLine;

// TODO: Add a type that supports X loyalty
type Loyalty = i32;

// TODO: Add a type that supports * for power and/or toughness
type Power = i32;
type Toughness = i32;

pub struct CardData {
    name: String,
    mana_cost: ManaCost,
    color_indicator: Option<ColorIdentity>,
    type_line: TypeLine,
    power_toughness: Option<(Power, Toughness)>,
    loyalty: Option<Loyalty>,
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
    }
}
