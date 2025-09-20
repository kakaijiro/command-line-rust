# Command Line Rust Projects

This repository contains a collection of command-line tools built with Rust.

## Projects

### hello
- **Description**: Basic Hello World program
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**: 
  - `assert_cmd` (dev-dependencies) - for testing

### echor
- **Description**: Echo command implementation
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**:
  - `clap` - command-line argument parsing

## Setup

Each project is an independent Rust project. You can build and run them with the following commands:

```bash
# hello project
cd hello
cargo run

# echor project
cd echor
cargo run
```

## Testing

Run tests for each project:

```bash
# hello project tests
cd hello
cargo test

# echor project tests
cd echor
cargo test
```

## Project Structure

```
command-line-rust/
├── hello/           # Hello World project
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   └── .gitignore
├── echor/           # Echo command project
│   ├── src/
│   ├── Cargo.toml
│   └── .gitignore
├── .gitignore       # Global .gitignore
└── README.md
```

## Development Environment

- Rust 1.70+
- Cargo
- Each project is managed as an independent Rust project

## License

This project is licensed under the MIT License.
