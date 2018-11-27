use serde::ser;
use std::{error, fmt};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ImpossibleKind {
    Bytes,
    UnitStruct,
    NewtypeVariant,
    Seq,
    Tuple,
    TupleStruct,
    TupleVariant,
    Map,
    Struct,
    StructVariant,
}

impl fmt::Display for ImpossibleKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ImpossibleKind::*;

        let name = match self {
            Bytes => "bytes",
            UnitStruct => "unit struct",
            NewtypeVariant => "newtype variant",
            Seq => "seq",
            Tuple => "tuple",
            TupleStruct => "tuple struct",
            TupleVariant => "tuple variant",
            Map => "map",
            Struct => "struct",
            StructVariant => "struct variant",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ErrorKind {
    Impossible(ImpossibleKind),
    Custom(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    fn impossible(kind: ImpossibleKind) -> Error {
        Error {
            kind: ErrorKind::Impossible(kind),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Impossible(kind) => write!(f, "impossible to serialize {}", kind),
            ErrorKind::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            kind: ErrorKind::Custom(msg.to_string()),
        }
    }
}

struct Serializer;

macro_rules! serialize_with_tostring {
    ($($Ty:ty => $meth:ident,)*) => {
        $(
            fn $meth(self, v: $Ty) -> Result<String, Error> {
                Ok(v.to_string())
            }
        )*
    };
}

type Impossible = ser::Impossible<String, Error>;

impl ser::Serializer for Serializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible;
    type SerializeTuple = Impossible;
    type SerializeTupleStruct = Impossible;
    type SerializeTupleVariant = Impossible;
    type SerializeMap = Impossible;
    type SerializeStruct = Impossible;
    type SerializeStructVariant = Impossible;

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
        Err(Error::impossible(ImpossibleKind::Bytes))
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
        Err(Error::impossible(ImpossibleKind::UnitStruct))
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
        Err(Error::impossible(ImpossibleKind::NewtypeVariant))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::Seq))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::Tuple))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::TupleStruct))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::TupleVariant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::Map))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::Struct))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Impossible, Error> {
        Err(Error::impossible(ImpossibleKind::StructVariant))
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T: ?Sized + ::core::fmt::Display>(self, _value: &T) -> Result<String, Error> {
        unimplemented!()
    }
}

pub fn to_string<T: ser::Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(Serializer)
}
