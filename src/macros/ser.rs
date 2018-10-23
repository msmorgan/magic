use serde::ser;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Impossible(&'static str),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Impossible(msg) => write!(f, "impossible to serialize {}", msg),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

struct Serializer;

macro_rules! serialize_with_tostring {
    ($($ty:ty => $meth:ident,)*) => {
        $(
            fn $meth(self, v: $ty)
                -> Result<<Self as ::serde::ser::Serializer>::Ok, <Self as ::serde::ser::Serializer>::Error> {
                Ok(v.to_string())
            }
        )*
    };
}

impl ser::Serializer for Serializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = ser::Impossible<String, Error>;
    type SerializeTuple = ser::Impossible<String, Error>;
    type SerializeTupleStruct = ser::Impossible<String, Error>;
    type SerializeTupleVariant = ser::Impossible<String, Error>;
    type SerializeMap = ser::Impossible<String, Error>;
    type SerializeStruct = ser::Impossible<String, Error>;
    type SerializeStructVariant = ser::Impossible<String, Error>;

    serialize_with_tostring! {
        bool => serialize_bool,
        i8 => serialize_i8,
        i16 => serialize_i16,
        i32 => serialize_i32,
        i64 => serialize_i64,
        u8 => serialize_u8,
        u16 => serialize_u16,
        u32 => serialize_u32,
        u64 => serialize_u64,
        f32 => serialize_f32,
        f64 => serialize_f64,
        char => serialize_char,
        &str => serialize_str,
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<String, Error> {
        Err(Error::Impossible("bytes"))
    }

    fn serialize_none(self) -> Result<String, Error> {
        Ok(String::new())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<String, Error> {
        Ok(String::new())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Error> {
        Err(Error::Impossible("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String, Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, Error> {
        Err(Error::Impossible("newtype variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Error::Impossible("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Error::Impossible("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::Impossible("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::Impossible("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Error::Impossible("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(Error::Impossible("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::Impossible("struct variant"))
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T: ?Sized + ::core::fmt::Display>(self, _value: &T) -> Result<String, Error> {
        unimplemented!()
    }
}

pub fn to_string<T: ser::Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(Serializer)
}
