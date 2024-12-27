//!
//! Contains newtype wrappers for Kafka integer types (`i8`, `i16`, `i32`, `i64`)
//! that implement Serde's `Serialize` and `Deserialize`. These types are
//! intended to represent the core Kafka wire-format integer primitives.
//!
//! For raw byte handling, consider using the [`KafkaSerializer`] and
//! [`KafkaDeserializer`] structs from `io.rs` in this crate.

use serde::{Deserialize, Serialize};

/// A newtype for Kafka's 8-bit integer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KafkaInt8(pub i8);

impl Serialize for KafkaInt8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // In high-level usage, this is just i8 -> i8, but if you wanted
        // to control exactly how the bytes are written, you'd integrate
        // with a custom Serializer. For now, we let Serde handle it.
        serializer.serialize_i8(self.0)
    }
}

impl<'de> Deserialize<'de> for KafkaInt8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i8::deserialize(deserializer)?;
        Ok(KafkaInt8(val))
    }
}

/// A newtype for Kafka's 16-bit integer (big-endian).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KafkaInt16(pub i16);

impl Serialize for KafkaInt16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i16(self.0)
    }
}

impl<'de> Deserialize<'de> for KafkaInt16 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i16::deserialize(deserializer)?;
        Ok(KafkaInt16(val))
    }
}

/// A newtype for Kafka's 32-bit integer (big-endian).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KafkaInt32(pub i32);

impl Serialize for KafkaInt32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl<'de> Deserialize<'de> for KafkaInt32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i32::deserialize(deserializer)?;
        Ok(KafkaInt32(val))
    }
}

/// A newtype for Kafka's 64-bit integer (big-endian).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KafkaInt64(pub i64);

impl Serialize for KafkaInt64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for KafkaInt64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i64::deserialize(deserializer)?;
        Ok(KafkaInt64(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KafkaDeserializer, KafkaSerializer};
    use std::io::Cursor;

    #[test]
    fn test_i32_custom_roundtrip() {
        let mut buffer = Vec::new();

        // Write a KafkaInt32 using your custom serializer
        {
            let mut ser = KafkaSerializer::new(&mut buffer);
            ser.write_i32(42).unwrap();
        }

        // Read it back with your custom deserializer
        {
            let mut de = KafkaDeserializer::new(Cursor::new(&buffer));
            let val = de.read_i32().unwrap();
            assert_eq!(val, 42);
        }
    }
}
