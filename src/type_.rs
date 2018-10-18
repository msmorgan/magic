use std::fmt;

macro_rules! impl_fromstr {
    ($($t:ty,)*) => {
        $(
            impl ::std::str::FromStr for $t {
                type Err = ::serde::de::value::Error;

                fn from_str(value: &str) -> Result<$t, Self::Err> {
                    use ::serde::de::{Deserialize, IntoDeserializer};

                    Self::deserialize(value.into_deserializer())
                }
            }
        )*
    };

    ($($t:ty),*) => {
        impl_fromstr!($($t,)*);
    };
}

/// Types (205.2a)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Artifact,
    Conspiracy,
    Creature,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

impl_display_with_serialize!(Type);
impl_fromstr_with_deserialize!(Type);

/// Subtypes (205.3)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Subtype {
    Artifact(ArtifactType),
    Enchantment(EnchantmentType),
    Land(LandType),
    Planeswalker(PlaneswalkerType),
    Spell(SpellType),
    Creature(CreatureType),
    Plane(PlanarType),
}

impl_display_with_serialize!(Subtype);
impl_fromstr_with_deserialize!(Subtype);

impl Subtype {
    pub fn valid_for(&self, type_: Type) -> bool {
        match self {
            Subtype::Artifact(_) => type_ == Type::Artifact,
            Subtype::Enchantment(_) => type_ == Type::Enchantment,
            Subtype::Land(_) => type_ == Type::Land,
            Subtype::Planeswalker(_) => type_ == Type::Planeswalker,
            Subtype::Spell(_) => type_ == Type::Instant || type_ == Type::Sorcery,
            Subtype::Creature(_) => type_ == Type::Creature || type_ == Type::Tribal,
            Subtype::Plane(_) => type_ == Type::Plane,
        }
    }
}

/// Artifact subtypes (205.3g)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactType {
    Clue,
    Contraption,
    Equipment,
    Fortification,
    Treasure,
    Vehicle,
}

impl_display_with_serialize!(ArtifactType);
impl_fromstr_with_deserialize!(ArtifactType);

/// Enchantment subtypes (205.3h)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnchantmentType {
    Aura,
    Cartouche,
    Curse,
    Saga,
    Shrine,
}

impl_display_with_serialize!(EnchantmentType);
impl_fromstr_with_deserialize!(EnchantmentType);

/// Land subtypes (205.3i)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandType {
    Desert,
    Forest,
    Gate,
    Island,
    Lair,
    Locus,
    Mine,
    Mountain,
    Plains,
    #[serde(rename = "Power-Plant")] PowerPlant,
    Swamp,
    Tower,
    #[serde(rename = "Urza's")] Urzas,
}

impl_display_with_serialize!(LandType);
impl_fromstr_with_deserialize!(LandType);

/// Planeswalker subtypes (205.3j)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlaneswalkerType {
    Ajani,
    Aminatou,
    Angrath,
    Arlinn,
    Ashiok,
    Bolas,
    Chandra,
    Dack,
    Daretti,
    Domri,
    Dovin,
    Elspeth,
    Estrid,
    Freyalise,
    Garruk,
    Gideon,
    Huatli,
    Jace,
    Jaya,
    Karn,
    Kaya,
    Kiora,
    Koth,
    Liliana,
    Nahiri,
    Narset,
    Nissa,
    Nixilis,
    Ral,
    Rowan,
    Saheeli,
    Samut,
    Sarkhan,
    Sorin,
    Tamiyo,
    Teferi,
    Tezzeret,
    Tibalt,
    Ugin,
    Venser,
    Vivien,
    Vraska,
    Will,
    Windgrace,
    Xenagos,
    Yanggu,
    Yanling,
}

impl_display_with_serialize!(PlaneswalkerType);
impl_fromstr_with_deserialize!(PlaneswalkerType);

/// Instant and sorcery subtypes (205.3k)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellType {
    Arcane,
    Trap,
}

impl_display_with_serialize!(SpellType);
impl_fromstr_with_deserialize!(SpellType);

/// Creature and tribal subtypes (205.3m)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreatureType {
    Advisor,
    Aetherborn,
    Ally,
    Angel,
    Antelope,
    Ape,
    Archer,
    Archon,
    Artificer,
    Assassin,
    #[serde(rename = "Assembly-Worker")] AssemblyWorker,
    Atog,
    Aurochs,
    Avatar,
    Azra,
    Badger,
    Barbarian,
    Basilisk,
    Bat,
    Bear,
    Beast,
    Beeble,
    Berserker,
    Bird,
    Blinkmoth,
    Boar,
    Bringer,
    Brushwagg,
    Camarid,
    Camel,
    Caribou,
    Carrier,
    Cat,
    Centaur,
    Cephalid,
    Chimera,
    Citizen,
    Cleric,
    Cockatrice,
    Construct,
    Coward,
    Crab,
    Crocodile,
    Cyclops,
    Dauthi,
    Demon,
    Deserter,
    Devil,
    Dinosaur,
    Djinn,
    Dragon,
    Drake,
    Dreadnought,
    Drone,
    Druid,
    Dryad,
    Dwarf,
    Efreet,
    Egg,
    Elder,
    Eldrazi,
    Elemental,
    Elephant,
    Elf,
    Elk,
    Eye,
    Faerie,
    Ferret,
    Fish,
    Flagbearer,
    Fox,
    Frog,
    Fungus,
    Gargoyle,
    Germ,
    Giant,
    Gnome,
    Goat,
    Goblin,
    God,
    Golem,
    Gorgon,
    Graveborn,
    Gremlin,
    Griffin,
    Hag,
    Harpy,
    Hellion,
    Hippo,
    Hippogriff,
    Homarid,
    Homunculus,
    Horror,
    Horse,
    Hound,
    Human,
    Hydra,
    Hyena,
    Illusion,
    Imp,
    Incarnation,
    Insect,
    Jackal,
    Jellyfish,
    Juggernaut,
    Kavu,
    Kirin,
    Kithkin,
    Knight,
    Kobold,
    Kor,
    Kraken,
    Lamia,
    Lammasu,
    Leech,
    Leviathan,
    Lhurgoyf,
    Licid,
    Lizard,
    Manticore,
    Masticore,
    Mercenary,
    Merfolk,
    Metathran,
    Minion,
    Minotaur,
    Mole,
    Monger,
    Mongoose,
    Monk,
    Monkey,
    Moonfolk,
    Mutant,
    Myr,
    Mystic,
    Naga,
    Nautilus,
    Nephilim,
    Nightmare,
    Nightstalker,
    Ninja,
    Noggle,
    Nomad,
    Nymph,
    Octopus,
    Ogre,
    Ooze,
    Orb,
    Orc,
    Orgg,
    Ouphe,
    Ox,
    Oyster,
    Pangolin,
    Pegasus,
    Pentavite,
    Pest,
    Phelddagrif,
    Phoenix,
    Pilot,
    Pincher,
    Pirate,
    Plant,
    Praetor,
    Prism,
    Processor,
    Rabbit,
    Rat,
    Rebel,
    Reflection,
    Rhino,
    Rigger,
    Rogue,
    Sable,
    Salamander,
    Samurai,
    Sand,
    Saproling,
    Satyr,
    Scarecrow,
    Scion,
    Scorpion,
    Scout,
    Serf,
    Serpent,
    Servo,
    Shade,
    Shaman,
    Shapeshifter,
    Sheep,
    Siren,
    Skeleton,
    Slith,
    Sliver,
    Slug,
    Snake,
    Soldier,
    Soltari,
    Spawn,
    Specter,
    Spellshaper,
    Sphinx,
    Spider,
    Spike,
    Spirit,
    Splinter,
    Sponge,
    Squid,
    Squirrel,
    Starfish,
    Surrakar,
    Survivor,
    Tetravite,
    Thalakos,
    Thopter,
    Thrull,
    Treefolk,
    Trilobite,
    Triskelavite,
    Troll,
    Turtle,
    Unicorn,
    Vampire,
    Vedalken,
    Viashino,
    Volver,
    Wall,
    Warrior,
    Weird,
    Werewolf,
    Whale,
    Wizard,
    Wolf,
    Wolverine,
    Wombat,
    Worm,
    Wraith,
    Wurm,
    Yeti,
    Zombie,
    Zubera,
}

impl_display_with_serialize!(CreatureType);
impl_fromstr_with_deserialize!(CreatureType);

/// Plane types (205.3n)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanarType {
    Alara,
    Arkhos,
    Azgol,
    Belenon,
    #[serde(rename = "Bolas's Meditation Realm")] BolassMeditationRealm,
    Dominaria,
    Equilor,
    Ergamon,
    Fabacin,
    Innistrad,
    Iquatana,
    Ir,
    Kaldheim,
    Kamigawa,
    Karsus,
    Kephalai,
    Kinshala,
    Kolbahan,
    Kyneth,
    Lorwyn,
    Luvion,
    Mercadia,
    Mirrodin,
    Moag,
    Mongseng,
    Muraganda,
    #[serde(rename = "New Phyrexia")] NewPhyrexia,
    Phyrexia,
    Pyrulea,
    Rabiah,
    Rath,
    Ravnica,
    Regatha,
    Segovia,
    #[serde(rename = "Serra's Realm")] SerrasRealm,
    Shadowmoor,
    Shandalar,
    Ulgrotha,
    Valla,
    Vryn,
    Wildfire,
    Xerex,
    Zendikar,
}

impl_display_with_serialize!(PlanarType);
impl_fromstr_with_deserialize!(PlanarType);

/// Supertypes (205.4c)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

impl_display_with_serialize!(Supertype);
impl_fromstr_with_deserialize!(Supertype);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn subtype_data() -> (String, Vec<Subtype>) {
        let json = r#"
[
    "Clue",
    "Saga",
    "Swamp",
    "Elspeth",
    "Arcane",
    "Vampire",
    "Lorwyn"
]
        "#.replace(char::is_whitespace, "");

        let subtypes = vec![
            Subtype::Artifact(ArtifactType::Clue),
            Subtype::Enchantment(EnchantmentType::Saga),
            Subtype::Land(LandType::Swamp),
            Subtype::Planeswalker(PlaneswalkerType::Elspeth),
            Subtype::Spell(SpellType::Arcane),
            Subtype::Creature(CreatureType::Vampire),
            Subtype::Plane(PlanarType::Lorwyn),
        ];

        (json, subtypes)
    }

    #[test]
    fn subtype_deserialize() {
        let (json, expected) = subtype_data();

        let subtypes: Vec<Subtype> = serde_json::from_str(&json).unwrap();

        assert_eq!(subtypes, expected);
    }

    #[test]
    fn subtype_serialize() {
        let (expected, subtypes) = subtype_data();

        let json = serde_json::to_string(&subtypes).unwrap();

        assert_eq!(json, expected);
    }

    #[test]
    fn subtype_to_string() {
        assert_eq!(Subtype::Creature(CreatureType::Merfolk).to_string(), "Merfolk");
    }

    #[test]
    fn subtype_parse() {
        assert_eq!(Subtype::Land(LandType::Mountain), "Mountain".parse::<Subtype>().unwrap());
    }

    #[test]
    fn bolas_realm_json() {
        assert_eq!(serde_json::to_string(&PlanarType::BolassMeditationRealm).unwrap(), "\"Bolas's Meditation Realm\"");
    }

    #[test]
    fn bolas_realm_to_string() {
        assert_eq!(PlanarType::BolassMeditationRealm.to_string(), "Bolas's Meditation Realm");
    }
}
