# Contributing to StreamXL

Thanks for your interest! StreamXL is a pure Rust + Python library for streaming XLSX files with high performance.

## Project layout

```
src/
├── lib.rs              # Main Rust library
├── reader.rs           # XLSX reading logic
├── writer.rs           # XLSX writing logic
└── utils.rs            # Shared utilities

python/
├── streamxl/           # Python bindings
└── __init__.py         # Public API

tests/                  # Integration tests
benches/                # Performance benchmarks
```

## Dev setup

### Prerequisites
- Rust 1.70+
- Python 3.9+
- maturin (for PyO3 bindings)

### Setup

```bash
# Install dependencies
pip install maturin pytest

# Build the extension
maturin develop --release

# Run tests
pytest -v
```

## Before opening a PR

- `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --all` and `pytest -v` pass
- New functionality has tests (prefer Rust-side tests for pure logic)
- Performance benchmarks show improvement or no regression
- Update `CHANGELOG.md` under "Unreleased"

## High-impact areas

- **Streaming writer optimization** — reduce memory footprint for large writes
- **Formula support** — expand formula parsing and evaluation
- **Conditional formatting** — implement styling pipelines
- **Performance benchmarks** — measure against openpyxl, pandas-xlsx

## License

By contributing, you agree your contributions are licensed under the [MIT License](./LICENSE).
