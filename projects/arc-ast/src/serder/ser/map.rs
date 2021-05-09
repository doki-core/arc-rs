use super::*;
use std::mem::swap;
use crate::value::Dict;


pub struct MapBuffer<'s> {
    ptr: &'s mut ReadableConfigSerializer,
    name: Option<&'static str>,
    k_slots: Vec<AST>,
    v_slots: Vec<AST>,
}

impl<'s> MapBuffer<'s> {
    pub fn new(ptr: &'s mut ReadableConfigSerializer, name: Option<&'static str>, _: usize) -> Self {
        Self { name, k_slots: vec![], ptr, v_slots: vec![] }
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for MapBuffer<'a> {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.ptr);
        self.k_slots.push(self.ptr.this.to_owned());
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ptr);
        self.v_slots.push(self.ptr.this.to_owned());
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        let mut map = Vec::with_capacity(self.k_slots.len());
        for (k, v) in self.k_slots.into_iter().zip(self.v_slots.into_iter()) {
            map.push(ASTKind::Pair(Box::new(k),Box::new(v)).into_node())
        }
        Ok(self.ptr.this = ASTKind::Dict(map).into_node())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
// Name[a -> b, c -> d]
impl<'a> ser::SerializeStruct for MapBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.ptr);
        self.k_slots.push(self.ptr.this.to_owned());
        value.serialize(&mut *self.ptr);
        self.v_slots.push(self.ptr.this.to_owned());
        Ok(())
    }

    fn end(self) -> Result<()> {
        let mut map = Vec::with_capacity(self.k_slots.len());
        for (k, v) in self.k_slots.into_iter().zip(self.v_slots.into_iter()) {
            map.push(ASTKind::Pair(Box::new(k),Box::new(v)).into_node())
        }
        Ok(self.ptr.this = ASTKind::Dict(map).into_node())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for MapBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
