# streamxl

**High-performance streaming XLSX reader for Python — powered by Rust**

[![CI](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml)
[![PyPI](https://img.shields.io/pypi/v/streamxl)](https://pypi.org/project/streamxl/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## The problem with existing XLSX libraries

| Library | 100k rows — time | 100k rows — memory | Notes |
|---------|------------------|--------------------|-------|
| openpyxl (full load) | 8.70s | **188 MB** | Loads entire workbook before row 1 |
| openpyxl (read_only) | 6.19s | 8.1 MB | Streaming, but slow XML parsing in Python |
| **streamxl** | **1.79s** | **21 MB** | Rust ZIP + XML engine, true streaming |

At 500k+ rows, openpyxl full load exceeds 1 GB of RAM and crashes on typical cloud instances. streamxl memory stays constant regardless of file size.

---

## Installation

```bash
pip install streamxl
```

Requires Python 3.8+. Pre-built wheels for Linux, macOS (Apple Silicon + Intel), Windows.

---

## Usage

### Basic — iterate rows

```python
import streamxl

for row in streamxl.read("data.xlsx"):
    print(row)
# ['Name', 'Age', 'Score']
# ['Alice', 30.0, 95.5]
# ...
```

### ETL pipeline — stream to CSV

```python
import csv, streamxl

with open("output.csv", "w", newline="") as f:
    writer = csv.writer(f)
    for row in streamxl.read("large.xlsx"):
        writer.writerow(row)
```

### Stream to pandas (chunk by chunk)

```python
import pandas as pd, streamxl

CHUNK = 10_000
rows = []
for row in streamxl.read("large.xlsx"):
    rows.append(row)
    if len(rows) == CHUNK:
        df = pd.DataFrame(rows)
        process(df)
        rows.clear()
```

---

## Cell value types

| XLSX type | Python type |
|-----------|-------------|
| String (shared string) | `str` |
| Number | `float` |
| Boolean | `bool` |
| Empty cell | `None` |

---

## How it works

streamxl is built in two layers:

```
streamxl.read("file.xlsx")
        │
        ▼
python/streamxl/api.py        Python iterator API
        │
        ▼
python/src/lib.rs             PyO3 bridge (zero-copy FFI)
        │
        ▼
core/src/stream.rs            Rust: orchestrates ZIP + XML parsing
   ├── zip_reader.rs          wraps the zip crate
   ├── shared_strings.rs      parses xl/sharedStrings.xml → Vec<String>
   └── sheet_parser.rs        streams <row> elements one at a time
```

1. The XLSX ZIP is opened and `sharedStrings.xml` is read once into memory (typically < 1 MB).
2. `sheet1.xml` is parsed as a stream — only one `<row>` is in memory at any time.
3. Cell values are resolved via index lookup into the shared string table.
4. PyO3 converts each Rust `Vec<CellValue>` to a Python `list` on demand.

See [docs/architecture.md](docs/architecture.md) for full details.

---

## Benchmarks

Run on Apple M-series, 16 GB RAM, Python 3.13, Rust 1.96, macOS.

### 50,000 rows × 5 columns

| Library | Time | Peak Memory |
|---------|------|-------------|
| openpyxl (full load) | 4.30s | ~85 MB |
| openpyxl (read_only) | 3.09s | 4.4 MB |
| **streamxl** | **0.88s** | **10.7 MB** |

### 100,000 rows × 5 columns

| Library | Time | Peak Memory |
|---------|------|-------------|
| openpyxl (full load) | 8.70s | **188 MB** |
| openpyxl (read_only) | 6.19s | 8.1 MB |
| **streamxl** | **1.79s** | **21 MB** |

**3.5× faster than openpyxl read_only. 5× faster than full load.**

Reproduce: `python benchmarks/openpyxl_vs_streamxl.py your_file.xlsx`

---

## Roadmap

- [x] Streaming XLSX reader (sheet1)
- [x] sharedStrings resolution
- [x] PyO3 Python bindings
- [x] Boolean and numeric cell types
- [ ] Multi-sheet support (`sheet` parameter)
- [ ] Date/datetime cell type
- [ ] Header row as dict keys
- [ ] PyPI wheel distribution (manylinux + macOS + Windows)

---

## Development

### Prerequisites

- Rust (stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- maturin: `pip install maturin`

### Build

```bash
git clone https://github.com/Mullassery/StreamXL.git
cd StreamXL
maturin develop
```

### Test

```bash
pip install pytest
pytest tests/
```

### Benchmark

```bash
bash scripts/benchmark.sh path/to/large_file.xlsx
```

---

## Contributing

PRs welcome. See [docs/design_decisions.md](docs/design_decisions.md) for context on key architectural choices before opening a large PR.

---

## License

MIT © Georgi Mullassery
