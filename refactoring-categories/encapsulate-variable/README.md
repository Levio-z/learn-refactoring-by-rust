# Encapsulate Variable Refactoring Example

This project demonstrates the **Encapsulate Variable** refactoring technique using Rust, showing different approaches to encapsulate mutable global state.

## Overview

The refactoring demonstrates how to transform a publicly exposed mutable global variable into a properly encapsulated one with controlled access through functions.

## Files Structure

- `s00_before_encapsulation.rs` - Original code with direct global variable access
- `s01_after_encapsulation.rs` - Basic encapsulation with getter/setter functions
- `s02_1_find_modify.rs` - Pre-step for clone encapsulation: identify and refactor modification logic
- `s02_2_clone_encapsulation.rs` - Clone encapsulation with explicit copying
- `s03_set_clone_encapsulation.rs` - Setter receives copy for safety
- `main.rs` - Main entry point running all examples

## Refactoring Steps

### Step 0: Before Encapsulation

- Global mutable state directly exposed
- Any code can read/modify without control
- No single point of control for invariants

### Step 1: Basic Encapsulation

- Make global variable private
- Provide getter and setter functions
- Control access to the variable reference

### Step 2: Advanced Encapsulation

- **2.1**: Identify and refactor modification logic
- **2.2**: Clone encapsulation - return data copies
- Prevent modifications from affecting shared state

### Step 3: Setter Receives Copy

- Setter functions receive copies of data
- Prevents source data modification issues
- Demonstrates Rust's ownership advantages

## Key Concepts

### Rust-Specific Advantages

- **Ownership System**: Natural protection against shared mutation
- **Move Semantics**: Setters naturally receive copies
- **Borrow Checker**: Prevents many common bugs
- **Clone Trait**: Explicit copying when needed

### Benefits of Encapsulation

- **Control**: Single point of access control
- **Safety**: Prevents unexpected modifications
- **Maintainability**: Easier to change implementation
- **Debugging**: Clearer data flow

## Running the Examples

```bash
cd refactoring-categories/encapsulate-variable
cargo run
```

Each example will demonstrate the encapsulation approach and print the result.

## Testing

```bash
cargo test
```

Tests verify that each encapsulation approach produces the expected output.

## Comparison with Other Languages

### Java/JavaScript Issues

- Need explicit cloning to prevent source data modification
- Shared references can lead to unexpected side effects
- More complex debugging due to shared state mutations

### Rust Advantages

- Ownership system naturally prevents many issues
- Move semantics make copying explicit
- Compile-time safety reduces runtime errors
- Clear separation between borrowing and ownership

## Further Reading

- "Refactoring: Improving the Design of Existing Code" by Martin Fowler
- Rust Book: Ownership and Borrowing chapters
- Rust by Example: Ownership and References

---

*This project is part of the "Learn Refactoring by Rust" series.*
