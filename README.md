# ANTLR Rust Language (Arithmetic)

A super simple language grammar parsing example with ANTLR4 rust target.

It can be used to kicksart a rust based parsing project using ANTLR4.

## Features

- A simple *arithmetic* grammar written in [ANTLR4](https://github.com/antlr/antlr4).
- Building it's ANTLR entites in rust by using [antlr-rust](https://github.com/rrevenantt/antlr4rust).
- Using the generated code to obtain [parse tree](https://en.wikipedia.org/wiki/Parse_tree).
- [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) implementation to create a basic [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

## Demo

```bash
> Write something like '(3+4)*2' to see the generated AST. ^C to exit:
> 23*(22+3)

Prog(
  [
    BinaryOperation(
      "*",
      IntVal(23),
      BinaryOperation(
        "+",
        IntVal(22),
        IntVal(3),
      ),
    ),
  ],
),
```

## Run Locally

Go to the project directory

```bash
cd antlr_rust_arithmetic
```

Download the [latest](https://github.com/rrevenantt/antlr4rust/releases) custom ANTLR4 build for rust:

```bash
curl -OL https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar
```

Build the ANTLR entites for rust target:

```bash
java -jar ./antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -visitor grammar/Lang.g4 -o src
```

Build and run the application

```bash
cargo run
```

## Acknowledgements

- Custom ANTLR4 build must to be used until the antlr-rust merge to the main antlr repo.

## Author

- [@yunusemredilber](https://www.github.com/yunusemredilber)
