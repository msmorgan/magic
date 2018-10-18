use indexmap::IndexSet;
use std::fmt;
use std::str::FromStr;

use type_::*;

const EM_DASH: &'static str = "—";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeLine {
    supertypes: IndexSet<Supertype>,
    types: IndexSet<Type>,
    subtypes: IndexSet<Subtype>,
}

impl TypeLine {
    pub fn new() -> TypeLine {
        TypeLine {
            supertypes: IndexSet::new(),
            types: IndexSet::new(),
            subtypes: IndexSet::new(),
        }
    }

    pub fn from_iters<Sup, T, Sub>(supertypes: Sup, types: T, subtypes: Sub) -> TypeLine
    where
        Sup: IntoIterator<Item = Supertype>,
        T: IntoIterator<Item = Type>,
        Sub: IntoIterator<Item = Subtype>,
    {
        use std::iter::FromIterator;

        TypeLine {
            supertypes: IndexSet::from_iter(supertypes),
            types: IndexSet::from_iter(types),
            subtypes: IndexSet::from_iter(subtypes),
        }
    }

    pub fn add_supertype(&mut self, supertype: Supertype) {
        self.supertypes.insert(supertype);
    }

    pub fn add_type(&mut self, type_: Type) {
        self.types.insert(type_);
    }

    pub fn add_subtype(&mut self, subtype: Subtype) {
        self.subtypes.insert(subtype);
    }

    pub fn has_supertype(&self, supertype: Supertype) -> bool {
        self.supertypes.contains(&supertype)
    }

    pub fn has_type(&self, type_: Type) -> bool {
        self.types.contains(&type_)
    }

    pub fn has_subtype(&self, subtype: Subtype) -> bool {
        self.subtypes.contains(&subtype)
    }

    pub fn remove_supertype(&mut self, supertype: Supertype) {
        self.supertypes.remove(&supertype);
    }

    pub fn remove_type(&mut self, type_: Type) {
        self.types.remove(&type_);
    }

    pub fn remove_subtype(&mut self, subtype: Subtype) {
        self.subtypes.remove(&subtype);
    }

    pub fn types_iter<'a>(&'a self) -> impl Iterator<Item = Type> + 'a {
        self.types.iter().cloned()
    }

    pub fn subtypes_iter<'a>(&'a self) -> impl Iterator<Item = Subtype> + 'a {
        self.subtypes.iter().cloned()
    }

    pub fn supertypes_iter<'a>(&'a self) -> impl Iterator<Item = Supertype> + 'a {
        self.supertypes.iter().cloned()
    }

    pub fn is_valid(&self) -> bool {
        !self.types.is_empty() && {
            self.subtypes_iter()
                .all(|sub| self.types_iter().any(|ty| sub.valid_for(ty)))
        }
    }
}

impl fmt::Display for TypeLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subtypes_len = self.subtypes.len();
        let subtypes_len = if subtypes_len > 0 {
            subtypes_len + 1
        } else {
            0
        };

        let mut parts = Vec::with_capacity(self.supertypes.len() + self.types.len() + subtypes_len);

        parts.extend(self.supertypes.iter().map(ToString::to_string));
        parts.extend(self.types.iter().map(ToString::to_string));
        if subtypes_len > 0 {
            parts.push(EM_DASH.to_string()); // em dash
            parts.extend(self.subtypes.iter().map(ToString::to_string));
        }

        f.write_str(&parts.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeLineReadError {
    NotAnEmDash(String),
    ExtraParts(String),
}

impl fmt::Display for TypeLineReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for TypeLine {
    type Err = TypeLineReadError;

    fn from_str(s: &str) -> Result<TypeLine, TypeLineReadError> {
        let mut halves = s.split(EM_DASH);

        let mut type_line = TypeLine::new();

        // pre dash
        {
            let mut parts = halves.next()
                .unwrap_or("")
                .split_whitespace()
                .filter(|s| !s.is_empty());

            let mut current = parts.next();

            while let Some(c) = current {
                if let Ok(supertype) = c.parse::<Supertype>() {
                    type_line.add_supertype(supertype);
                    current = parts.next();
                } else {
                    break;
                }
            }

            while let Some(c) = current {
                if let Ok(type_) = c.parse::<Type>() {
                    type_line.add_type(type_);
                    current = parts.next();
                } else {
                    break;
                }
            }

            if current.is_some() {
                return Err(TypeLineReadError::ExtraParts(current.unwrap().to_string()));
            }
        }

        // post dash
        {
            if let Some(post_dash) = halves.next() {
                let post_dash = post_dash.trim();
                if let Ok(subtype) = post_dash.parse::<Subtype>() {
                    type_line.add_subtype(subtype);
                } else {
                    let mut parts = post_dash.split_whitespace()
                        .filter(|s| !s.is_empty());

                    let mut current = parts.next();

                    while let Some(c) = current {
                        if let Ok(subtype) = c.parse::<Subtype>() {
                            type_line.add_subtype(subtype);
                            current = parts.next();
                        } else {
                            break;
                        }
                    }

                    if current.is_some() {
                        return Err(TypeLineReadError::ExtraParts(current.unwrap().to_string()));
                    }
                }
            }
        }

        Ok(type_line)
    }
}

impl_deserialize_with_fromstr!(TypeLine);
impl_serialize_with_tostring!(TypeLine);

macro_rules! type_line {
    ($($supertype:ident)* ; $($type_:ident)* ; $($subtype:ident)*) => {
        {
            let mut type_line = TypeLine::new();

            $(
                type_line.add_supertype($crate::type_::Supertype::$supertype);
            )*

            $(
                type_line.add_type($crate::type_::Type::$type_);
            )*

            {
                use $crate::type_::{
                    CreatureType::*,
                    ArtifactType::*,
                    EnchantmentType::*,
                    SpellType::*,
                    PlanarType::*,
                    LandType::*,
                    PlaneswalkerType::*,
                };

                $(
                    type_line.add_subtype($subtype.into_subtype());
                )*
            }

            type_line
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_type_line_data() -> (TypeLine, String) {
        let type_line = TypeLine::from_iters(
            [Supertype::Legendary].iter().cloned(),
            [Type::Creature].iter().cloned(),
            [
                Subtype::Creature(CreatureType::Merfolk),
                Subtype::Creature(CreatureType::Wizard),
            ].iter().cloned(),
        );

        let string = format!("Legendary Creature {} Merfolk Wizard", EM_DASH);

        (type_line, string)
    }

    #[test]
    fn type_line_macro() {
        let type_line = type_line!(Legendary; Enchantment Creature; Human Wizard);
        assert_eq!(type_line.to_string(), format!("Legendary Enchantment Creature {} Human Wizard", EM_DASH));
    }

    #[test]
    fn serialize_type_line() {
        let (type_line, expected) = get_type_line_data();
        assert_eq!(type_line.to_string(), expected);
    }

    #[test]
    fn deserialize_type_line() {
        let (expected, string) = get_type_line_data();

        assert_eq!(string.parse::<TypeLine>().unwrap(), expected);
    }

    #[test]
    fn bolas_realm() {
        let type_line = {
            let mut res = TypeLine::new();
            res.add_type(Type::Plane);
            res.add_subtype(Subtype::Plane(PlanarType::BolassMeditationRealm));
            res
        };

        let string = format!("Plane {} Bolas's Meditation Realm", EM_DASH);
        assert!(type_line.is_valid());

        let serialized = type_line.to_string();
        assert_eq!(serialized, string);

        let deserialized = string.parse::<TypeLine>().unwrap();
        assert_eq!(deserialized, type_line);
    }
}
