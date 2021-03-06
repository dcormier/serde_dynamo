use super::{AttributeValue, Error, Item, Result, Serializer};
use serde::{ser, Serialize};

pub struct SerializerMap {
    item: Item,
}

impl SerializerMap {
    pub fn new(len: Option<usize>) -> Self {
        let item = if let Some(len) = len {
            Item::with_capacity(len)
        } else {
            Item::new()
        };
        SerializerMap { item }
    }
}

impl<'a> ser::SerializeMap for SerializerMap {
    type Ok = AttributeValue;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = key.serialize(MapKeySerializer)?;
        let value = value.serialize(Serializer)?;
        self.item.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(AttributeValue {
            m: Some(self.item),
            ..AttributeValue::default()
        })
    }
}

struct MapKeySerializer;

impl<'a> ser::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!()
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unreachable!()
    }
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unreachable!()
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unreachable!()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!()
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unreachable!()
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }
}

impl<'a> ser::SerializeSeq for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl<'a> ser::SerializeTuple for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl<'a> ser::SerializeTupleStruct for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
impl<'a> ser::SerializeTupleVariant for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a> ser::SerializeStructVariant for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a> ser::SerializeMap for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        _key: &K,
        _value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<'a> ser::SerializeStruct for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
