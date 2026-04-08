# Contributing to tumodori

Thanks for your interest in contributing to tumodori!

## Getting started

1. Fork the repository
2. Clone your fork:

```bash
git clone https://github.com/<your-username>/tumodori.git
cd tumodori
```

3. Create a feature branch:

```bash
git checkout -b my-feature
```

4. Make your changes and ensure everything compiles:

```bash
cargo build
cargo test
cargo clippy
```

5. Commit your changes with a descriptive message
6. Push to your fork and open a Pull Request

## Development requirements

- Rust stable (latest)
- Cargo

## Code style

- Follow standard Rust conventions (`rustfmt`)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Write tests for new functionality

## Reporting bugs

Open an issue with:

- Steps to reproduce
- Expected behavior
- Actual behavior
- Terminal emulator and OS

## Suggesting features

Open an issue describing the feature, why it would be useful, and any implementation ideas you have.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
