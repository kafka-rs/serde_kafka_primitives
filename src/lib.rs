//! Top-level docs about `serde_kafka_primitives`
//! This library implements Serde-based logic for Kafka wire protocol primitives.

mod int;
// mod varint;
// mod string;
// mod array;
pub mod io;

// Expose these modules/types publicly so users can import them directly.
pub use int::{KafkaInt16, KafkaInt32, KafkaInt64, KafkaInt8};
// pub use varint::{KafkaVarInt, KafkaVarLong};
// pub use string::{KafkaString, KafkaNullableString, KafkaCompactString};
// pub use array::{KafkaArray, KafkaCompactArray};

// If you create custom serializer/deserializer structs:
pub use io::{KafkaDeserializer, KafkaSerializer};
