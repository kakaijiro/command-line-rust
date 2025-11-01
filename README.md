# Command Line Rust Projects

This repository contains a collection of command-line tools built with Rust, inspired by the Unix command-line utilities.

## Projects

### hello
- **Description**: Basic Hello World program
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**: 
  - `assert_cmd` (dev-dependencies, v1) - for testing

### echor
- **Description**: Echo command implementation
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**:
  - `clap` (v4) - command-line argument parsing
  - `assert_cmd` (dev-dependencies, v2) - for testing
  - `predicates` (dev-dependencies, v3) - for testing assertions
  - `regex` (dev-dependencies, v1) - for regex matching in tests

### catr
- **Description**: Cat command implementation with line numbering features
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**:
  - `clap` (v4) - command-line argument parsing
  - `assert_cmd` (dev-dependencies, v2) - for testing
  - `predicates` (dev-dependencies, v3) - for testing assertions
  - `rand` (dev-dependencies, v0.9) - for test data generation

### headr
- **Description**: Head command implementation for displaying the first lines/bytes of files
- **Version**: 0.1.0
- **Edition**: 2024
- **Dependencies**:
  - `clap` (v4) - command-line argument parsing
  - `assert_cmd` (dev-dependencies, v2) - for testing
  - `predicates` (dev-dependencies, v3) - for testing assertions
  - `rand` (dev-dependencies, v0.9) - for test data generation

### wcr
- **Description**: Word count (wc) command implementation for counting lines, words, bytes, and characters
- **Version**: 0.1.0
- **Edition**: 2024
- **Features**:
  - Count lines (`-l, --lines`)
  - Count words (`-w, --words`)
  - Count bytes (`-c, --bytes`)
  - Count characters (`-m, --chars`)
  - Display totals for multiple files
  - Support for wildcard patterns (e.g., `*.txt`)
  - Support for standard input (`-` or no argument)
- **Dependencies**:
  - `clap` (v4) - command-line argument parsing
  - `assert_cmd` (dev-dependencies, v2) - for testing
  - `predicates` (dev-dependencies, v3) - for testing assertions
  - `rand` (dev-dependencies, v0.9) - for test data generation

## Setup

Each project is an independent Rust project. You can build and run them with the following commands:

```bash
# hello project
cd hello
cargo run

# echor project
cd echor
cargo run

# catr project
cd catr
cargo run

# headr project
cd headr
cargo run

# wcr project
cd wcr
cargo run
# Example: cargo run -- tests/inputs/*.txt
# Example: cargo run -- -l -w tests/inputs/fox.txt
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

# catr project tests
cd catr
cargo test

# headr project tests
cd headr
cargo test

# wcr project tests
cd wcr
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
├── catr/            # Cat command project
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   └── .gitignore
├── headr/           # Head command project
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   └── .gitignore
├── wcr/             # Word count (wc) command project
│   ├── src/
│   ├── tests/
│   │   ├── cli.rs
│   │   ├── inputs/
│   │   └── expected/
│   ├── Cargo.toml
│   └── .gitignore
├── .gitignore       # Global .gitignore
└── README.md
```

## Development Environment

- Rust 1.70+ (or Rust Edition 2024 compatible version)
- Cargo
- Each project is managed as an independent Rust project

## Usage Examples

### wcr - Word Count

```bash
cd wcr

# Count lines, words, and bytes (default)
cargo run -- tests/inputs/fox.txt

# Count only lines
cargo run -- --lines tests/inputs/fox.txt

# Count words and bytes
cargo run -- -w -c tests/inputs/fox.txt

# Count characters instead of bytes
cargo run -- --chars tests/inputs/fox.txt

# Process multiple files with wildcards
cargo run -- tests/inputs/*.txt

# Read from standard input
echo "Hello world" | cargo run --
```

## License

This project is licensed under the MIT License.
