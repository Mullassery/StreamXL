# CLAUDE.md — streamxl

This file tells Claude Code how to work in this repository correctly.

## Build system

```bash
# Always build with maturin, never cargo build --workspace
maturin develop          # editable install (dev)
maturin build --release  # optimised wheel → target/wheels/

# Rust core only (no Python headers needed)
cargo build --manifest-path core/Cargo.toml
cargo test  --manifest-path core/Cargo.toml
```

**Do NOT run `cargo build --workspace`** — `python/` is a standalone workspace that requires Python headers and must be built via maturin.

## Test

```bash
maturin develop   # must rebuild after any Rust change
pytest tests/ -v
```

## Repository layout

- `core/src/` — Rust library: `sheet_parser.rs`, `shared_strings.rs`, `stream.rs`, `zip_reader.rs`
- `python/src/lib.rs` — PyO3 bridge (the `.so` extension module)
- `python/streamxl/` — Python package: `__init__.py`, `api.py`, `core.py`
- `pyproject.toml` — maturin config; `manifest-path = "python/Cargo.toml"`
- `Cargo.toml` — workspace root; members = `["core"]` only

## Critical constraints

### read_event() not read_event_into()

`SheetParser` uses `Reader<&[u8]>` and must call `read_event()`. **Do not switch to `read_event_into(&mut buf)`** — it silently produces wrong results for string cells when the reader is over a byte-slice reference.

### inlineStr vs sharedString

openpyxl writes strings as `t="inlineStr"` with `<is><t>text</t></is>`, not as SST (`t="s"`) references. Both code paths exist in `sheet_parser.rs` and both are required. Do not remove either.

### Workspace split

`python/Cargo.toml` starts with `[workspace]` to make it a standalone workspace. This is required so maturin can build it independently from the root workspace. Without it, `cargo` and `maturin` conflict.

### Bool conversion in PyO3 0.23

`bool.into_pyobject(py)` returns `Borrowed<'_, '_, PyBool>`, not `Bound`. Use `.as_any().clone().unbind()` — not `.into_any().unbind()` — to convert it to `PyObject`.

## Cell types

| XLSX `t` attr | Rust variant | Python type |
|---------------|--------------|-------------|
| `s` | `CellValue::String` | `str` |
| `inlineStr` | `CellValue::String` | `str` |
| `b` | `CellValue::Bool` | `bool` |
| `n` / absent | `CellValue::Number` | `float` |
| empty | `CellValue::Empty` | `None` |

## Not yet implemented

- Multi-sheet support (`sheet=` parameter)
- Date/datetime cell type
- `as_dict=True`
