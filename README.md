# chamox – Obfuscation Macro for Rust

`chamox` is a Rust procedural macro designed to generate obfuscated and meaningless code blocks to make reverse engineering more complex. This can be useful in scenarios where code security matters, such as protecting intellectual property or adding layers of obfuscation to compiled binaries.

## Features

- 🌀 Inserts random, meaningless calculations and control flows
- 🔀 Generates misleading code structures to confuse decompilers
- 🎭 Makes static analysis more difficult without modifying actual logic

## Example Usage

```rust
use chamox::obfuscate;

#[obfuscate]
fn hidden_logic() -> i32 {
    // This function's implementation will be obfuscated
    42
}
```

## Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
chamox = "0.1"
```

## How It Works
The `#[obfuscate]` macro injects meaningless operations within your function, making it harder for static analysis tools and decompilers to reconstruct the original logic.

## Security Notice
While this macro makes reverse engineering more complex, it **does not provide real security**. Do not rely on it for cryptographic protection.

## License

This project is licensed under MIT or Apache-2.0.
