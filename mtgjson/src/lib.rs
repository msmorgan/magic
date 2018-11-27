use chrono::NaiveDate;
use hex::{FromHex, ToHex};
use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt, str::FromStr};

use magic_core::{
    macros::{impl_deserialize_with_fromstr, impl_serialize_with_tostring},
    Color,
};

#[derive(Debug, Copy, Clone)]
pub struct Hex<T>(pub T);

impl<'de, T> Deserialize<'de> for Hex<T>
where
    T: FromHex,
    T::Error: ToString,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).and_then(|hex| {
            T::from_hex(hex)
                .map(Hex)
                .map_err(|err| de::Error::custom(err.to_string()))
        })
    }
}

impl<T> Serialize for Hex<T>
where
    T: ToHex,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::error::Error;

        let mut buf = String::new();
        self.0
            .write_hex(&mut buf)
            .map_err(|e| ser::Error::custom(e.description()))?;
        serializer.serialize_str(&buf)
    }
}

#[derive(Debug, Copy, Clone)]
struct Abbreviated<T>(pub T);

impl fmt::Display for Abbreviated<Color> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Abbreviated<Option<Color>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "C"),
            Some(c) => fmt::Display::fmt(&c, f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAbbreviatedColorError {
    _priv: (),
}

impl fmt::Display for ParseAbbreviatedColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "provided string was a recognized abbreviated color")
    }
}

impl FromStr for Abbreviated<Color> {
    type Err = ParseAbbreviatedColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Color::*;

        match s {
            "W" => Ok(White),
            "U" => Ok(Blue),
            "B" => Ok(Black),
            "R" => Ok(Red),
            "G" => Ok(Green),
            _ => Err(ParseAbbreviatedColorError { _priv: () }),
        }
        .map(|c| Abbreviated(c))
    }
}

impl FromStr for Abbreviated<Option<Color>> {
    type Err = ParseAbbreviatedColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Abbreviated(None)),
            s => Abbreviated::<Color>::from_str(s).map(|c| Abbreviated(Some(c.0))),
        }
    }
}

impl_serialize_with_tostring!(Abbreviated<Color>);
impl_deserialize_with_fromstr!(Abbreviated<Color>);
impl_serialize_with_tostring!(Abbreviated<Option<Color>>);
impl_deserialize_with_fromstr!(Abbreviated<Option<Color>>);

#[derive(Debug)]
struct ShortenedDate(pub NaiveDate);

impl Serialize for ShortenedDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShortenedDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|mut string| {
            match string.len() {
                10 => Ok(()),
                7 => {
                    string.push_str("-01");
                    Ok(())
                }
                4 => {
                    string.push_str("-01-01");
                    Ok(())
                }
                _ => Err(de::Error::custom("invalid date length")),
            }?;

            match string.parse() {
                Ok(date) => Ok(ShortenedDate(date)),
                Err(err) => Err(de::Error::custom(err.to_string())),
            }
        })
    }
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == Default::default()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    #[serde(default, skip_serializing_if = "is_default")]
    alternative_names: Vec<String>,
    block: Option<String>,
    booster: Option<Vec<BoosterItem>>,
    border: String,
    cards: Vec<Card>,
    code: String,
    essential_magic_code: Option<String>,
    gatherer_code: Option<String>,
    #[serde(rename = "isMCISet", default)]
    is_mci_set: bool,
    magic_cards_info_code: Option<String>,
    #[serde(default)]
    use_magic_rarities_number: bool,
    magic_rarities_codes: Option<Vec<String>>,
    #[serde(rename = "mkm_id")]
    mkm_id: Option<i32>,
    #[serde(rename = "mkm_name")]
    mkm_name: Option<String>,
    name: String,
    old_code: Option<String>,
    #[serde(default)]
    online_only: bool,
    release_date: NaiveDate,
    language: Option<String>,
    translations: Option<Translations>,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoosterItem {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    artist: String,
    border: Option<String>,
    cmc: f32,
    #[serde(default)]
    color_identity: Vec<Abbreviated<Color>>,
    #[serde(default)]
    colors: Vec<Color>,
    flavor: Option<String>,
    foreign_names: Option<Vec<ForeignCard>>,
    hand: Option<i32>,
    id: Hex<[u8; 20]>,
    image_name: String,
    layout: String,
    legalities: Option<Vec<FormatLegality>>,
    life: Option<i32>,
    loyalty: Option<i32>,
    mana_cost: Option<String>,
    mci_number: Option<String>,
    multiverseid: Option<i32>,
    name: String,
    names: Option<Vec<String>>,
    number: Option<String>,
    original_text: Option<String>,
    power: Option<String>,
    printings: Vec<String>,
    rarity: String,
    release_date: Option<ShortenedDate>,
    #[serde(default)]
    reserved: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    rulings: Vec<Ruling>,
    source: Option<String>,
    #[serde(default)]
    starter: bool,
    #[serde(default)]
    subtypes: Vec<String>,
    #[serde(default)]
    supertypes: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    text: String,
    #[serde(default)]
    timeshifted: bool,
    toughness: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default)]
    types: Vec<String>,
    variations: Option<Vec<i32>>,
    watermark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignCard {
    language: String,
    multiverseid: Option<i32>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatLegality {
    format: String,
    legality: Legality,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Legality {
    Legal,
    Restricted,
    Banned,
}

impl fmt::Display for Legality {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ruling {
    date: NaiveDate,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Translations {
    cn: Option<String>,
    de: Option<String>,
    es: Option<String>,
    fr: Option<String>,
    it: Option<String>,
    jp: Option<String>,
    ko: Option<String>,
    pt: Option<String>,
    ru: Option<String>,
    tw: Option<String>,
    zh_hans: Option<String>,
    zh_hant: Option<String>,
}

fn read_all_sets<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<Set>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let mut set_map = serde_json::from_str::<HashMap<String, Set>>(&json)?;
    let sets = set_map.drain().map(|(_k, v)| v).collect();
    Ok(sets)
}
