use super::Serializer;
use crate::error::Error;
use magnus::{RHash, Symbol, Value};
use serde::{ser::SerializeStructVariant, Serialize};

pub struct StructVariantSerializer {
    variant: &'static str,
    hash: RHash,
}

impl StructVariantSerializer {
    pub fn new(variant: &'static str, hash: RHash) -> StructVariantSerializer {
        StructVariantSerializer { variant, hash }
    }
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Value: Serialize + ?Sized>(
        &mut self,
        name: &'static str,
        value: &Value,
    ) -> Result<(), Self::Error> {
        self.hash
            .aset(Symbol::new(name), value.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let hash = RHash::new();
        hash.aset(self.variant, self.hash)?;
        Ok(hash.into())
    }
}