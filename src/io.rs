//! This contains the Kafka-specific I/O logic for reading and writing
//! raw bytes according to Kafka's wire protocol. It provides helper structs
//! `KafkaSerializer` and `KafkaDeserializer` for use in your custom
//! Serde implementations, or directly for low-level byte operations.

use std::io::{self, Read, Write};

/// A serializer for writing Kafka wire data to an underlying `Write` stream.
///
/// # Example
/// ```
/// use serde_kafka_primitives::io::KafkaSerializer;
/// use std::io::Cursor;
///
/// let mut buffer = Vec::new();
/// {
///     let mut ser = KafkaSerializer::new(&mut buffer);
///     // Write 42 (as i32, big-endian) to the buffer.
///     ser.write_i32(42).unwrap();
/// }
///
/// // buffer now contains the big-endian bytes for 42.
/// assert_eq!(buffer.len(), 4);
///
/// // We can verify the contents manually if needed:
/// // i32 42 in big-endian is [0, 0, 0, 42].
/// assert_eq!(buffer, vec![0x00, 0x00, 0x00, 0x2A]);
/// ```
pub struct KafkaSerializer<W: Write> {
    writer: W,
}

impl<W: Write> KafkaSerializer<W> {
    /// Create a new `KafkaSerializer` that writes to the given `Write` implementor.
    pub fn new(writer: W) -> Self {
        KafkaSerializer { writer }
    }

    /// Writes a `i32` in **big-endian** format to the underlying stream.
    pub fn write_i32(&mut self, val: i32) -> io::Result<()> {
        self.writer.write_all(&val.to_be_bytes())
    }

    /// Writes a `i16` in **big-endian** format to the underlying stream.
    pub fn write_i16(&mut self, val: i16) -> io::Result<()> {
        self.writer.write_all(&val.to_be_bytes())
    }

    /// Writes an `i64` in **big-endian** format to the underlying stream.
    pub fn write_i64(&mut self, val: i64) -> io::Result<()> {
        self.writer.write_all(&val.to_be_bytes())
    }

    /// Writes an `i8` (which is just one byte).
    pub fn write_i8(&mut self, val: i8) -> io::Result<()> {
        self.writer.write_all(&[val as u8])
    }

    // TODO: Add more specialized write methods (varint, varlong, strings, arrays) as needed.
}

/// A deserializer for reading Kafka wire data from an underlying `Read` stream.
///
/// # Example
/// ```
/// use serde_kafka_primitives::io::KafkaDeserializer;
/// use std::io::Cursor;
///
/// // The big-endian bytes for i32 42.
/// let data = vec![0x00, 0x00, 0x00, 0x2A];
/// let mut de = KafkaDeserializer::new(Cursor::new(data));
///
/// // Read i32 in big-endian, expect 42.
/// let value = de.read_i32().unwrap();
/// assert_eq!(value, 42);
/// ```
pub struct KafkaDeserializer<R: Read> {
    reader: R,
}

impl<R: Read> KafkaDeserializer<R> {
    /// Create a new `KafkaDeserializer` that reads from the given `Read` implementor.
    pub fn new(reader: R) -> Self {
        KafkaDeserializer { reader }
    }

    /// Reads a `i32` in **big-endian** format from the underlying stream.
    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    /// Reads a `i16` in **big-endian** format from the underlying stream.
    pub fn read_i16(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    /// Reads an `i64` in **big-endian** format from the underlying stream.
    pub fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    /// Reads an `i8`, which is just a single byte interpreted as `i8`.
    pub fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    // TODO: Add specialized read methods (varint, varlong, strings, arrays) as needed.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_i32_roundtrip() {
        let mut buffer = Vec::new();

        // Write i32
        {
            let mut ser = KafkaSerializer::new(&mut buffer);
            ser.write_i32(42).unwrap();
        }

        // Read i32
        {
            let mut de = KafkaDeserializer::new(Cursor::new(&buffer));
            let val = de.read_i32().unwrap();
            assert_eq!(val, 42);
        }
    }

    #[test]
    fn test_i8_roundtrip() {
        let mut buffer = Vec::new();

        // Write i8
        {
            let mut ser = KafkaSerializer::new(&mut buffer);
            ser.write_i8(-5).unwrap(); // 0xFB
        }

        // Read i8
        {
            let mut de = KafkaDeserializer::new(Cursor::new(&buffer));
            let val = de.read_i8().unwrap();
            assert_eq!(val, -5);
        }
    }
}
