use serde::{Deserialize, Serialize};

use magic_core::Color;
use uuid::Uuid;

pub type Uri = String;
pub type Date = String;
pub type MimeType = String;
pub type Encoding = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum Object {
    BulkData(BulkData),
    Card(Card),
    CardFace(CardFace),
    CardSymbol(CardSymbol),
    Catalog(Catalog),
    Error(Error),
    List(List),
    RelatedCard(RelatedCard),
    Ruling(Ruling),
    Set(Set),
}

macro_rules! object_conversions {
    ($($Variant:ident => $is_meth:ident, $as_meth:ident, $into_meth:ident;)*) => {
        $(
            pub fn $is_meth(&self) -> bool {
                match self {
                    Object::$Variant(_) => true,
                    _ => false,
                }
            }

            pub fn $as_meth(&self) -> Option<&$Variant> {
                match self {
                    Object::$Variant(x) => Some(x),
                    _ => None,
                }
            }

            pub fn $into_meth(self) -> Option<$Variant> {
                match self {
                    Object::$Variant(x) => Some(x),
                    _ => None,
                }
            }
        )*
    }
}

impl Object {
    object_conversions! {
        BulkData    => is_bulk_data,    as_bulk_data,    into_bulk_data;
        Card        => is_card,         as_card,         into_card;
        CardFace    => is_card_face,    as_card_face,    into_card_face;
        CardSymbol  => is_card_symbol,  as_card_symbol,  into_card_symbol;
        Catalog     => is_catalog,      as_catalog,      into_catalog;
        Error       => is_error,        as_error,        into_error;
        List        => is_list,         as_list,         into_list;
        RelatedCard => is_related_card, as_related_card, into_related_card;
        Ruling      => is_ruling,       as_ruling,       into_ruling;
        Set         => is_set,          as_set,          into_set;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    status: u16,
    code: String,
    details: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    warnings: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    data: Vec<Object>,
    has_more: bool,
    next_page: Option<Uri>,
    total_cards: Option<usize>,
    warnings: Option<Vec<String>>,
}

impl List {
    pub fn data(&self) -> impl Iterator<Item = &Object> {
        self.data.iter()
    }

    pub fn bulk_data(&self) -> impl Iterator<Item = &BulkData> {
        self.data.iter().filter_map(|obj| match obj {
            Object::BulkData(bulk_data) => Some(bulk_data),
            _ => None,
        })
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.data.iter().filter_map(Object::as_card)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    uri: Uri,
    total_values: usize,
    data: Vec<String>,
}

impl Catalog {
    pub fn data(&self) -> &[String] {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSymbol {
    symbol: String,
    loose_variant: Option<String>,
    english: String,
    transposable: bool,
    represents_mana: bool,
    cmc: Option<f32>,
    appears_in_mana_costs: bool,
    funny: bool,
    colors: Vec<Color>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    Meld,
    Leveler,
    Saga,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Frame {
    #[serde(rename = "1993")]
    Original,
    #[serde(rename = "1997")]
    Updated,
    #[serde(rename = "2003")]
    Modern,
    #[serde(rename = "2015")]
    Frontier,
    #[serde(rename = "future")]
    Future,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BorderColor {
    Black,
    Borderless,
    Gold,
    Silver,
    White,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    id: Uuid,
    oracle_id: Uuid,
    multiverse_ids: Option<Vec<u32>>,
    mtgo_id: Option<u32>,
    mtgo_foil_id: Option<u32>,
    uri: Uri,
    scryfall_uri: Uri,
    prints_search_uri: Uri,
    rulings_uri: Uri,
    arena_id: Option<u32>,

    name: String,
    layout: Layout,
    cmc: f32,
    type_line: Option<String>,
    oracle_text: Option<String>,
    mana_cost: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    life_modifier: Option<String>,
    hand_modifier: Option<String>,
    colors: Option<Vec<Color>>,
    color_indicator: Option<Vec<Color>>,
    color_identity: Vec<Color>,
    all_parts: Option<Vec<Object>>,
    card_faces: Option<Vec<Object>>,
    legalities: Legalities,
    reserved: bool,
    edhrec_rank: Option<u32>,

    set: String,
    set_name: String,
    collector_number: String,
    set_search_uri: Uri,
    scryfall_set_uri: Uri,
    image_uris: Option<ImageUris>,
    highres_image: bool,
    reprint: bool,
    digital: bool,
    rarity: String,
    flavor_text: Option<String>,
    artist: Option<String>,
    illustration_id: Option<Uuid>,
    frame: Frame,
    full_art: bool,
    watermark: Option<String>,
    border_color: BorderColor,
    story_spotlight_number: Option<u32>,
    story_spotlight_uri: Option<Uri>,
    timeshifted: bool,
    colorshifted: bool,
    futureshifted: bool,

    tix: Option<String>,
    usd: Option<String>,
    eur: Option<String>,

    related_uris: Option<RelatedUris>,
    purchase_uris: Option<PurchaseUris>,
}

impl Card {
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedCard {
    id: Uuid,
    name: String,
    uri: Uri,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardFace {
    name: String,
    type_line: String,
    oracle_text: Option<String>,
    mana_cost: String,
    colors: Option<Vec<Color>>,
    color_indicator: Option<Vec<Color>>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    flavor_text: Option<String>,
    illustration_id: Option<Uuid>,
    image_uris: Option<ImageUris>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUris {
    png: Uri,
    border_crop: Uri,
    art_crop: Uri,
    large: Uri,
    normal: Uri,
    small: Uri,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Legalities {
    standard: Legality,
    frontier: Legality,
    modern: Legality,
    pauper: Legality,
    legacy: Legality,
    penny: Legality,
    vintage: Legality,
    commander: Legality,
    #[serde(rename = "duel")]
    duel_commander: Legality,
    #[serde(rename = "1v1")]
    commander_1v1: Legality,
    future: Legality,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Core,
    Expansion,
    Masters,
    Masterpiece,
    FromTheVault,
    PremiumDeck,
    DuelDeck,
    Commander,
    Planechase,
    Conspiracy,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    Box,
    Promo,
    Token,
    Memorabilia,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    code: String,
    mtgo_code: String,
    name: String,
    set_type: SetType,
    released_at: Option<Date>,
    block_code: Option<String>,
    block: Option<String>,
    parent_set_code: Option<String>,
    card_count: usize,
    digital: bool,
    foil: bool,
    icon_svg_uri: Uri,
    search_uri: Uri,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RulingSource {
    Wotc,
    Scryfall,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ruling {
    source: RulingSource,
    published_at: Date,
    comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedUris {
    gatherer: Option<Uri>,
    edhrec: Option<Uri>,
    tcgplayer_decks: Option<Uri>,
    mtgtop8: Option<Uri>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseUris {
    amazon: Option<Uri>,
    ebay: Option<Uri>,
    tcgplayer: Option<Uri>,
    magiccardmarket: Option<Uri>,
    cardhoarder: Option<Uri>,
    card_kingdom: Option<Uri>,
    mtgo_traders: Option<Uri>,
    coolstuffinc: Option<Uri>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BulkData {
    id: Uuid,
    #[serde(rename = "type")]
    type_: String,
    name: String,
    description: String,
    permalink_uri: Uri,
    updated_at: Date,
    size: Option<usize>,
    compressed_size: Option<usize>,
    content_type: MimeType,
    content_encoding: Encoding,
}

impl BulkData {
    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn permalink_uri(&self) -> &Uri {
        &self.permalink_uri
    }
}
