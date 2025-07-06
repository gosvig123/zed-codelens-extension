# CodeLens Extension for Zed

A Zed extension that provides reference counts for symbols in JavaScript, TypeScript, and Python files.

## Features

- Shows reference counts for:
  - Function definitions
  - Variable declarations
  - Class definitions
  - TypeScript interfaces and types
  - Python classes and functions

## Installation

1. Clone this repository
2. Install Rust if not already installed
3. Build the extension:
   ```bash
   cargo build --target wasm32-wasi --release
   ```
4. Install the extension in Zed as a dev extension

## Usage

Once installed, the extension will automatically analyze your JavaScript, TypeScript, and Python files and show reference counts next to symbol definitions.

## Development

### Building

```bash
cargo build --target wasm32-wasi --release
```

### Testing

The extension includes basic pattern matching for symbol detection. Future versions will integrate with Tree-sitter for more accurate parsing.

## Limitations

- Currently uses simple pattern matching instead of full AST parsing
- Does not handle complex cases like destructuring or dynamic references
- Reference counting is limited to the current file scope

## Future Improvements

- Integration with Tree-sitter for accurate AST parsing
- Cross-file reference counting
- Support for more languages
- Integration with LSP for semantic analysis