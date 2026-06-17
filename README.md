# streamxl

**Read Microsoft Excel files (`.xlsx`) row by row without loading them into memory — powered by Rust.**

[![CI](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com/Mullassery/StreamXL/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Python](https://img.shields.io/badge/python-3.9%2B-blue)](https://pypi.org/project/streamxl/)
[![Rust](https://img.shields.io/badge/rust-1.96%2B-orange)](https://www.rust-lang.org/)

If your team exports data from Microsoft Excel, Google Sheets, LibreOffice Calc, or any other spreadsheet tool as `.xlsx`, streamxl lets you process those files in Python without loading the entire workbook into memory. It streams the sheet XML one row at a time and runs 4–5× faster than openpyxl across all file sizes.

---

## Install

```bash
pip install streamxl
# or
uv add streamxl
```

**Wheels:** Linux (x86_64, aarch64) · macOS (Apple Silicon, Intel) · Windows (x86_64)

<details>
<summary>Other install options</summary>

**One-liner (auto-detects uv or pip, builds from source if no wheel exists):**
```bash
curl -sSf https://raw.githubusercontent.com/Mullassery/StreamXL/main/scripts/install.sh | sh
```

**Latest from GitHub:**
```bash
pip install git+https://github.com/Mullassery/StreamXL.git
```

**From source:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Rust, if not installed
pip install maturin
git clone https://github.com/Mullassery/StreamXL.git
cd StreamXL
maturin develop --release
```
</details>

**Requires:** Python 3.9+ · Rust 1.70+ (source builds only)

---

## Usage

### Iterate rows from an Excel file

```python
import streamxl

for row in streamxl.read("report.xlsx"):
    print(row)
# ['Name', 'Age', 'Score']
# ['Alice', 30.0, 95.5]
# ...
```

Works with any `.xlsx` file — exports from Microsoft Excel, Google Sheets ("Download as .xlsx"), LibreOffice Calc, Numbers, or any tool that writes the Office Open XML format.

### Stream a large Excel export to CSV

```python
import csv, streamxl

with open("output.csv", "w", newline="") as f:
    writer = csv.writer(f)
    for row in streamxl.read("excel_export.xlsx"):
        writer.writerow(row)
```

### Process an Excel workbook in chunks with pandas

```python
import pandas as pd, streamxl

CHUNK = 10_000
rows = []
for row in streamxl.read("large_report.xlsx"):
    rows.append(row)
    if len(rows) == CHUNK:
        df = pd.DataFrame(rows)
        process(df)
        rows.clear()
```

`streamxl.stream()` is an alias for `streamxl.read()` — use whichever reads better in your context.

---

## Why not just use openpyxl?

openpyxl is the standard Python library for reading Excel files, but it loads the entire workbook into memory. At 250k rows it approaches 1 GB RAM and crashes on typical cloud instances. streamxl processes the same file in 68 MB.

Benchmarked on Apple Silicon, Python 3.13, Rust 1.96 — 10 mixed-type columns:

| Rows | streamxl | openpyxl read_only | openpyxl full load | Speedup |
|------|----------|--------------------|--------------------|---------|
| 10,000 | **0.40s** · 2.8 MB | 1.52s · 1.5 MB | 1.94s · 38 MB | **3.8×** |
| 50,000 | **1.81s** · 13.8 MB | 7.72s · 4.3 MB | 9.83s · 186 MB | **4.3×** |
| 100,000 | **3.59s** · 27.5 MB | 15.80s · 8.1 MB | 19.77s · 373 MB | **4.4×** |
| 250,000 | **9.04s** · 68.7 MB | 40.46s · 19.6 MB | 50.67s · **911 MB** | **4.5×** |

Throughput: ~27,000 rows/sec regardless of file size.

Full results and reproduction steps: [`benchmarks/results.md`](benchmarks/results.md)

```bash
python benchmarks/openpyxl_vs_streamxl.py your_file.xlsx
```

---

## Cell value types

Excel cells are mapped to Python types as follows:

| Excel cell type | Python type | Notes |
|----------------|-------------|-------|
| Shared string (`t="s"`) | `str` | Resolved from sharedStrings.xml |
| Inline string (`t="inlineStr"`) | `str` | Read directly from sheet XML |
| Number (`t="n"` or default) | `float` | All numeric values returned as float |
| Boolean (`t="b"`) | `bool` | `"1"` → `True`, `"0"` → `False` |
| Empty cell | `None` | Cell absent or blank |

---

## Roadmap

- [x] Streaming XLSX reader (sheet1)
- [x] sharedStrings resolution
- [x] inlineStr cell type support
- [x] Boolean, numeric, and string cell types
- [x] pip and uv installable wheel
- [ ] Multi-sheet support (`sheet="SheetName"` parameter)
- [ ] Date/datetime cell type
- [ ] Header row as dict keys (`as_dict=True`)
- [ ] PyPI wheel distribution (manylinux + macOS + Windows)

---

## How it works

`.xlsx` is a ZIP archive containing XML files. streamxl opens the ZIP, loads `sharedStrings.xml` once (the dictionary Excel uses to deduplicate repeated strings), then event-streams `sheet1.xml` via `quick-xml` — so only one row ever lives in memory at a time.

```
streamxl.read("file.xlsx")
        │
        ▼
python/streamxl/api.py        Python iterator API
        │
        ▼
python/src/lib.rs             Python bridge (zero-copy FFI)
        │
        ▼
core/src/stream.rs            Rust: orchestrates ZIP + XML parsing
   ├── zip_reader.rs          wraps the zip crate for entry access
   ├── shared_strings.rs      parses xl/sharedStrings.xml → Vec<String>
   └── sheet_parser.rs        streams <row> elements one at a time
```

See [docs/architecture.md](docs/architecture.md) for full details.

---

## Repository layout

```
streamxl/
├── core/                    # Rust engine (ZIP + XML streaming)
│   └── src/
│       ├── lib.rs
│       ├── zip_reader.rs
│       ├── sheet_parser.rs  # inlineStr + sharedString + bool/number parsing
│       ├── shared_strings.rs
│       └── stream.rs
├── python/                  # Python API layer
│   ├── src/lib.rs           # Rust ↔ Python bridge
│   └── streamxl/
│       ├── __init__.py
│       ├── api.py           # streamxl.read() / streamxl.stream()
│       └── core.py
├── benchmarks/              # openpyxl vs streamxl comparison scripts
├── tests/                   # pytest test suite
├── examples/                # ETL, CSV export, memory benchmark
├── docs/                    # Architecture, API spec, XLSX format notes
├── scripts/                 # build.sh, benchmark.sh
├── pyproject.toml           # maturin build config
└── Cargo.toml               # Rust workspace root
```

---

## Development

```bash
git clone https://github.com/Mullassery/StreamXL.git
cd StreamXL
maturin develop
```

**Test:**
```bash
pip install pytest
pytest tests/ -v
```

**Benchmark:**
```bash
bash scripts/benchmark.sh path/to/large_file.xlsx
```

Read [docs/design_decisions.md](docs/design_decisions.md) before opening a large PR.

---

## Contributing

PRs welcome. See [docs/design_decisions.md](docs/design_decisions.md) for context on key architectural choices.

---

## License

MIT © Georgi Mullassery
