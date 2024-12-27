//! This module defines newtype wrappers for Kafka integer types (`i8`, `i16`,
//! `i32`, `i64`). These wrappers implement Serde's `Serialize` and `Deserialize`
//! traits, providing a Kafka-compatible representation of the core wire-format
//! integer primitives.
//!
//! # Important
//! - Although these types implement `Serialize` and `Deserialize`, the actual
//!   **byte order** you produce or consume will depend on the underlying
//!   format and serializer. For direct, low-level I/O in **big-endian** format,
//!   see the [`KafkaSerializer`] and [`KafkaDeserializer`] structs in
//!   `io.rs` within this crate.

use serde::{Deserialize, Serialize};

/// A newtype for Kafka's 8-bit integer (`i8`).
///
/// # Serialization
/// - Uses standard `i8` Serde serialization by default.
/// - For big-endian byte-level I/O, consider using
///   [`KafkaSerializer::write_i8`][crate::KafkaSerializer::write_i8] and
///   [`KafkaDeserializer::read_i8`][crate::KafkaDeserializer::read_i8].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KafkaInt8(pub i8);

impl Serialize for KafkaInt8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // We delegate to Serde's built-in i8 handling here.
        serializer.serialize_i8(self.0)
    }
}

impl<'de> Deserialize<'de> for KafkaInt8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // We simply deserialize as an i8, then wrap it.
        let val = i8::deserialize(deserializer)?;
        Ok(KafkaInt8(val))
    }
}

/// A newtype for Kafka's 16-bit integer (`i16`), typically represented in big-endian.
///
/// # Serialization
/// - Uses standard `i16` Serde serialization by default.
/// - For low-level byte I/O in Kafka's expected big-endian order, use
///   [`KafkaSerializer::write_i16`][crate::KafkaSerializer::write_i16] and
///   [`KafkaDeserializer::read_i16`][crate::KafkaDeserializer::read_i16].
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

/// A newtype for Kafka's 32-bit integer (`i32`), typically represented in big-endian.
///
/// # Serialization
/// - Uses standard `i32` Serde serialization by default.
/// - For low-level byte I/O in Kafka's expected big-endian order, use
///   [`KafkaSerializer::write_i32`][crate::KafkaSerializer::write_i32] and
///   [`KafkaDeserializer::read_i32`][crate::KafkaDeserializer::read_i32].
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

/// A newtype for Kafka's 64-bit integer (`i64`), typically represented in big-endian.
///
/// # Serialization
/// - Uses standard `i64` Serde serialization by default.
/// - For low-level byte I/O in Kafka's expected big-endian order, use
///   [`KafkaSerializer::write_i64`][crate::KafkaSerializer::write_i64] and
///   [`KafkaDeserializer::read_i64`][crate::KafkaDeserializer::read_i64].
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
    //! This module contains unit tests that verify round-trip behavior
    //! using the custom serializer/deserializer found in `io.rs`.
    //!
    //! The big-endian encoding is tested by writing integers with
    //! [`KafkaSerializer`][crate::KafkaSerializer] and reading them back with
    //! [`KafkaDeserializer`][crate::KafkaDeserializer].

    use super::*;
    use crate::{KafkaDeserializer, KafkaSerializer};
    use std::io::Cursor;

    #[test]
    fn test_i32_custom_roundtrip() {
        // We use a Vec<u8> to capture bytes in memory.
        let mut buffer = Vec::new();

        // Write a 32-bit integer (42) in big-endian format.
        {
            let mut ser = KafkaSerializer::new(&mut buffer);
            ser.write_i32(42).expect("Failed to write i32");
        }

        // Read it back and verify correctness.
        {
            let mut de = KafkaDeserializer::new(Cursor::new(&buffer));
            let val = de.read_i32().expect("Failed to read i32");
            assert_eq!(val, 42);
        }
    }
}
