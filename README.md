
# serde_kafka_primitives

A minimal crate providing **Serde-compatible** types for **Kafka wire protocol primitives**.  
This library aims to offer a building block for higher-level Kafka clients, test harnesses, or any other Rust applications that need to encode or decode raw Kafka protocol data.

## Status

> **⚠️ Alpha Release:** This crate is under heavy development. Basic Kafka primitive types (e.g., `KafkaInt32`, `KafkaInt64`) exist, but the feature set may be incomplete. Expect potentially breaking changes as we refine the API.

## Features

- **Serde Integration**: Implements `Serialize` and `Deserialize` for Kafka’s common wire-level integer and string types.  
- **Modular**: Focuses solely on low-level primitives so you can reuse them in your own crates.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
serde_kafka_primitives = { git = "https://github.com/kafka-rs/serde_kafka_primitives", branch = "main" }
```
*(Once the crate is published on [crates.io](https://crates.io/), you can specify a version instead.)*

## Usage

Here’s a quick example showing how you might leverage `KafkaInt32` in a Serde-powered struct.  
*(Note that real Kafka wire usage typically involves reading/writing raw bytes in big-endian; this example is simplified for demonstration.)*

```rust
use serde::{Serialize, Deserialize};
use serde_kafka_primitives::KafkaInt32;

/// A simple struct illustrating how you might embed Kafka wire primitives.
#[derive(Serialize, Deserialize, Debug)]
pub struct MyKafkaMessage {
    pub some_field: KafkaInt32,
    // ... add more fields as you implement additional primitives
}

fn main() {
    let msg = MyKafkaMessage {
        some_field: KafkaInt32(42),
    };

    // Serialize to JSON (just as an example of Serde usage)
    let json_str = serde_json::to_string(&msg).unwrap();
    println!("Serialized to JSON: {}", json_str);

    // Deserialize from JSON back to our struct
    let deserialized: MyKafkaMessage = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized struct: {:?}", deserialized);
}
```

### Extending Primitives

**`serde_kafka_primitives`** offers multiple Kafka types out of the box:
- `KafkaInt8`  *(coming soon)*  
- `KafkaInt16`  *(coming soon)*  
- `KafkaInt32`  *(coming soon)*  
- `KafkaInt64`  *(coming soon)*  
- `KafkaVarInt` *(coming soon)*  
- `KafkaString` and `KafkaNullableString` *(planned)*  
- `KafkaCompactString` *(planned)*

Feel free to open an issue or contribute a pull request if you need new Kafka primitives.

## Roadmap

1. **Primitives Completion**: Add missing types (VarInt, VarLong, strings, arrays).  
2. **I/O Streaming**: Provide an optional custom `Serializer`/`Deserializer` that directly reads/writes big-endian data from an `io::Read` and `io::Write`.  
3. **Integration Testing**: Add integration tests comparing against official Kafka message specs.  

## Contributing

We welcome contributions to the serde_kafka_primitives project! Whether it's reporting bugs, proposing new features, improving documentation, or contributing code, your help is greatly appreciated. Here's how you can contribute:

1. **Fork the Repository**: Start by forking the serde_kafka_primitives repository to your own GitHub account. This will create a copy of the repository that you can modify without affecting the original project.  
2. **Create a Branch**: In your forked repository, create a new branch for the changes you want to make. This helps keep your changes separate from other changes and makes it easier to merge your changes later.  
3. **Make Your Changes**: Make your changes in the new branch. This could be fixing a bug, adding a new feature, improving documentation, or any other changes you think would improve the project.  
4. **Test Your Changes**: Make sure your changes work as expected and don't introduce any new bugs. If the project has a test suite, make sure your changes pass all the tests.  
5. **Submit a Pull Request**: Once you're happy with your changes, submit a pull request to merge your branch into the main serde_kafka_primitives repository. In your pull request, describe the changes you made and why you think they should be included in the project.  
6. **Address Review Feedback**: After you submit a pull request, other contributors to the project may review your changes and provide feedback. Be prepared to make additional changes or answer questions about your changes.

Remember, contributions to open source projects like serde_kafka_primitives are a collaborative effort. Be respectful and patient with other contributors, and remember that everyone is working together to improve the project.

Thank you for your interest in contributing to serde_kafka_primitives!

## License

Licensed under the [Apache License, Version 2.0](LICENSE)  
```


