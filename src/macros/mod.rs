pub mod ser;

macro_rules! impl_display_with_debug {
    ($($t:ty,)*) => {
        $(
            impl ::std::fmt::Display for $t {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    ::std::fmt::Debug::fmt(self, f)
                }
            }
        )*
    };

    ($($t:ty),*) => {
        impl_display_with_debug!($($t,)*);
    };
}

macro_rules! impl_display_with_serialize {
    ($($t:ty,)*) => {
        $(
            impl ::std::fmt::Display for $t {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "{}", $crate::macros::ser::to_string(self).unwrap())
                }
            }
        )*
    };

    ($($t:ty),*) => {
        impl_display_with_serialize!($($t,)*);
    }
}

macro_rules! impl_fromstr_with_deserialize {
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
        impl_fromstr_with_deserialize!($($t,)*);
    };
}

macro_rules! impl_deserialize_with_fromstr {
    ($($t:ty,)*) => {
        $(
            impl<'de> ::serde::de::Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<$t, D::Error>
                    where D: ::serde::de::Deserializer<'de> {
                    let s: &'de str = ::serde::de::Deserialize::deserialize(deserializer)?;
                    s.parse::<$t>().map_err(::serde::de::Error::custom)
                }
            }

        )*
    };

    ($($t:ty),*) => {
        impl_deserialize_with_fromstr!($($t,)*);
    };
}

macro_rules! impl_serialize_with_tostring {
    ($($t:ty,)*) => {
        $(
            impl ::serde::ser::Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: ::serde::ser::Serializer {
                    serializer.serialize_str(&self.to_string())
                }
            }
        )*
    };

    ($($t:ty),*) => {
        impl_serialize_with_tostring!($($t,)*);
    };
}
