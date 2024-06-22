# tree-sitter

- [docs](https://tree-sitter.github.io/tree-sitter/)
  - [using parsers](https://tree-sitter.github.io/tree-sitter/using-parsers)
  - [creating parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers)
  - [syntax highlighting](https://tree-sitter.github.io/tree-sitter/syntax-highlighting)
- github: [tree-sitter](https://github.com/tree-sitter/tree-sitter)

## tree-sitter-cli

- crates: [tree-sitter-cli](https://crates.io/crates/tree-sitter-cli)

Install CLI tool:

```bash
cargo install tree-sitter-cli
```

### Project Setup

```bash
mkdir tree-sitter-${YOUR_LANGUAGE_NAME}
cd tree-sitter-${YOUR_LANGUAGE_NAME}

npm init
vi grammar.js
tree-sitter generate
```

