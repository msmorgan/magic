pub mod ser;

#[macro_export]
macro_rules! impl_display_with_debug {
    ($Ty:ty) => {
        impl ::std::fmt::Display for $Ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, f)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_with_serialize {
    ($Ty:ty) => {
        impl ::std::fmt::Display for $Ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", $crate::ser::to_string(self).unwrap())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_fromstr_with_deserialize {
    ($Ty:ty) => {
        impl ::std::str::FromStr for $Ty {
            type Err = ::serde::de::value::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                use ::serde::de::{Deserialize, IntoDeserializer};

                Self::deserialize(value.into_deserializer())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deserialize_with_fromstr {
    ($Ty:ty) => {
        impl<'de> ::serde::de::Deserialize<'de> for $Ty {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $Ty;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        f.write_str(concat!("a valid ", stringify!($Ty)))
                    }

                    fn visit_str<E>(self, value: &str) -> ::std::result::Result<$Ty, E>
                    where
                        E: ::serde::de::Error,
                    {
                        value.parse().map_err(|_| {
                            ::serde::de::Error::invalid_value(
                                ::serde::de::Unexpected::Str(value),
                                &self,
                            )
                        })
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }
    };

}

#[macro_export]
macro_rules! impl_serialize_with_tostring {
    ($Ty:ty) => {
        impl ::serde::ser::Serialize for $Ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
}
