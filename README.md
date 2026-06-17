# streamxl

**A Python library for reading and writing Microsoft Excel files (`.xlsx`) — powered by Rust.**

[![CI](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/StreamXL/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://github.com/Mullassery/StreamXL/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Python](https://img.shields.io/badge/python-3.9%2B-blue)](https://pypi.org/project/streamxl/)
[![Rust](https://img.shields.io/badge/rust-1.96%2B-orange)](https://www.rust-lang.org/)

streamxl is a Python library for reading and writing `.xlsx` spreadsheets — files used by Microsoft Excel, Google Sheets, LibreOffice Calc, and any tool that uses the Office Open XML format. It streams row by row on both read and write, so you never load the entire workbook into memory, and runs 4–5× faster than openpyxl on read and ~10× faster on write.

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

## Reading

### Iterate rows from an Excel file

```python
import streamxl

for row in streamxl.read("report.xlsx"):
    print(row)
# ['Name', 'Joined', 'Score']
# ['Alice', datetime.date(2024, 1, 15), 95.5]
# ...
```

Works with any `.xlsx` file — exports from Microsoft Excel, Google Sheets ("Download as .xlsx"), LibreOffice Calc, Numbers, or any tool that writes the Office Open XML format.

### Read a specific sheet

```python
# List all sheets in the file
print(streamxl.sheets("report.xlsx"))
# ['Summary', 'Data', 'Config']

# Read by sheet name
for row in streamxl.read("report.xlsx", sheet="Data"):
    print(row)
```

### Read rows as dicts

```python
for row in streamxl.read("report.xlsx", as_dict=True):
    print(row)
# {'Name': 'Alice', 'Joined': datetime.date(2024, 1, 15), 'Score': 95.5}
# {'Name': 'Bob',   'Joined': datetime.date(2023, 8, 3),  'Score': 88.0}
```

The first row is treated as the header and consumed — subsequent rows are yielded as dicts keyed by header values.

### Filter columns

```python
# By column index (0-based)
for row in streamxl.read("report.xlsx", columns=[0, 2]):
    print(row)  # only Name and Score

# By column name (works with or without as_dict)
for row in streamxl.read("report.xlsx", as_dict=True, columns=["Name", "Score"]):
    print(row)  # {'Name': ..., 'Score': ...}
```

### Stream to CSV

```python
import csv, streamxl

with open("output.csv", "w", newline="") as f:
    writer = csv.writer(f)
    for row in streamxl.read("excel_export.xlsx"):
        writer.writerow(row)
```

### Process in chunks with pandas

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

`streamxl.stream()` is an alias for `streamxl.read()`.

---

## Writing

### Write rows in one call

```python
import datetime, streamxl

streamxl.write("report.xlsx", [
    ["Name", "Joined", "Score", "Active"],
    ["Alice", datetime.date(2024, 1, 15), 95.5, True],
    ["Bob",   datetime.date(2023, 8, 3),  88.0, False],
])
```

Supported cell types: `str`, `int`, `float`, `bool`, `None`, `datetime.date`, `datetime.datetime`.

### Write multiple sheets

```python
import streamxl

with streamxl.writer("report.xlsx") as w:
    # Sheet1 (default)
    w.write_row(["Name", "Score"])
    w.write_row(["Alice", 95.5])

    w.add_sheet("Summary")
    w.write_row(["Metric", "Value"])
    w.write_row(["Total", 1])

    w.add_sheet("Config")
    w.write_row(["Key", "Value"])
    w.write_row(["version", "1.0"])
# file is finalised on __exit__
```

### ETL: read one Excel file, transform, write another

```python
import streamxl

with streamxl.writer("output.xlsx") as w:
    w.write_row(["name", "amount_usd"])
    for row in streamxl.read("source.xlsx"):
        name, amount_gbp = row[0], row[3]
        w.write_row([name, amount_gbp * 1.27])
```

---

## Why not just use openpyxl?

openpyxl full load approaches 1 GB RAM at 250k rows and crashes on typical cloud instances. openpyxl `write_only` mode is safer but still pure Python. streamxl does both in Rust.

All benchmarks measured on Apple Silicon (M-series), Python 3.13, Rust 1.96, 10 mixed-type columns.

**Read** — streamxl vs openpyxl:

| Rows | streamxl | openpyxl read_only | openpyxl full load | Speedup |
|------|----------|--------------------|--------------------|---------|
| 10,000 | **29ms** · 4.3 MB | 1.31s · 2.9 MB | 1.25s · 34 MB | **44×** |
| 50,000 | **149ms** · 21.6 MB | 7.02s · 13.3 MB | 6.76s · 166 MB | **47×** |
| 100,000 | **308ms** · 43.2 MB | 14.33s · 26.3 MB | 13.34s · 332 MB | **46×** |
| 250,000 | **777ms** · 108 MB | 35.99s · 66 MB | 34.63s · **811 MB** | **46×** |

Read throughput: ~320,000 rows/sec. Full results: [`benchmarks/results.md`](benchmarks/results.md)

**Write** — streamxl vs openpyxl write_only:

| Rows | streamxl | openpyxl write_only | Speedup |
|------|----------|---------------------|---------|
| 10,000 | **12ms** · 0.1 MB | 102ms · 0.2 MB | **8.8×** |
| 50,000 | **44ms** · 0.4 MB | 476ms · 1.0 MB | **10.9×** |
| 100,000 | **90ms** · 0.8 MB | 958ms · 2.0 MB | **10.7×** |
| 250,000 | **230ms** · 1.9 MB | 2.41s · 5.1 MB | **10.5×** |

Write throughput: ~1,000,000 rows/sec.

```bash
python benchmarks/openpyxl_vs_streamxl.py your_file.xlsx
python benchmarks/openpyxl_vs_streamxl_write.py
```

---

## Cell value types

**Reading** — Excel cells are mapped to Python types:

| Excel cell type | Python type | Notes |
|----------------|-------------|-------|
| Shared string (`t="s"`) | `str` | Resolved from sharedStrings.xml |
| Inline string (`t="inlineStr"`) | `str` | Read directly from sheet XML |
| Number (`t="n"` or default) | `float` | All numeric values returned as float |
| Boolean (`t="b"`) | `bool` | `"1"` → `True`, `"0"` → `False` |
| Date (numFmtId 14–17) | `datetime.date` | Excel serial converted to Python date |
| Datetime (numFmtId 22) | `datetime.datetime` | Excel serial + time fraction |
| Empty cell | `None` | Cell absent or blank |

**Writing** — Python types are mapped to Excel cells:

| Python type | Excel cell | Notes |
|-------------|------------|-------|
| `str` | Shared string | Deduplicated via SST |
| `int` / `float` | Number | |
| `bool` | Boolean | |
| `datetime.date` | Date | Written with numFmtId 14 (mm-dd-yy) |
| `datetime.datetime` | Datetime | Written with numFmtId 22 (m/d/yy h:mm) |
| `None` | Empty cell | |

---

## API reference

```python
# Read
streamxl.read(path, sheet=None, as_dict=False, columns=None)
streamxl.stream(path)           # alias for read()
streamxl.sheets(path)           # → list of sheet names

# Write (all rows at once)
streamxl.write(path, rows)

# Write (streaming / multi-sheet)
w = streamxl.writer(path)
w.write_row(row)                # write one row to the current sheet
w.add_sheet(name)               # finalise current sheet, start a new one
w.close()                       # finalise and write the file
# or use as a context manager (close called automatically on __exit__)
```

---

## Roadmap

- [ ] PyPI manylinux + macOS + Windows wheel distribution
- [ ] Append mode — open an existing XLSX and add rows
- [ ] Cell formatting on write — bold headers, number formats
- [ ] `streamxl.read_all()` → list of `{sheet_name: [rows]}` for all sheets

---

## How it works

`.xlsx` is a ZIP archive of XML files. On **read**, streamxl loads `sharedStrings.xml` and `styles.xml` once, then event-streams the target sheet via `quick-xml` — one row in memory at a time. Numeric cells with a date style are converted to Python `datetime` objects using a Julian Day Number algorithm. On **write**, rows are encoded directly to XML in Rust as they arrive, with strings deduplicated into a shared string table; the ZIP is assembled and flushed to disk on close.

```
streamxl.read("file.xlsx")          streamxl.write("file.xlsx", rows)
        │                                    │
        ▼                                    ▼
python/streamxl/api.py              python/streamxl/api.py
        │                                    │
        ▼                                    ▼
python/src/lib.rs  (PyO3 bridge)    python/src/lib.rs  (PyO3 bridge)
        │                                    │
        ▼                                    ▼
core/src/stream.rs                  core/src/writer.rs
   ├── workbook.rs   sheet names         └── ZIP + XML + SST in Rust
   ├── styles.rs     date detection
   ├── dates.rs      serial conversion
   ├── shared_strings.rs
   └── sheet_parser.rs
```

See [docs/architecture.md](docs/architecture.md) for full details.

---

## Repository layout

```
streamxl/
├── core/                    # Rust engine
│   └── src/
│       ├── stream.rs        # read orchestration + sheet selection
│       ├── writer.rs        # write: multi-sheet XML + ZIP assembly
│       ├── sheet_parser.rs  # streaming XML row parser
│       ├── shared_strings.rs
│       ├── styles.rs        # date format detection from styles.xml
│       ├── workbook.rs      # sheet name + relationship parsing
│       ├── dates.rs         # Excel serial ↔ Python date math
│       └── zip_reader.rs
├── python/                  # Python API + PyO3 bridge
│   ├── src/lib.rs
│   └── streamxl/
│       ├── __init__.py
│       ├── api.py           # read(), stream(), write(), writer(), sheets()
│       └── core.py
├── benchmarks/              # read and write benchmarks vs openpyxl
├── tests/                   # pytest suite (46 tests)
├── examples/
├── docs/
├── scripts/
├── pyproject.toml
└── Cargo.toml
```

---

## Development

```bash
git clone https://github.com/Mullassery/StreamXL.git
cd StreamXL
maturin develop --release
```

**Test:**
```bash
pip install pytest
pytest tests/ -v
```

**Benchmark read:**
```bash
python benchmarks/openpyxl_vs_streamxl.py path/to/file.xlsx
```

**Benchmark write:**
```bash
python benchmarks/openpyxl_vs_streamxl_write.py
```

Read [docs/design_decisions.md](docs/design_decisions.md) before opening a large PR.

---

## Contributing

PRs welcome. See [docs/design_decisions.md](docs/design_decisions.md) for context on key architectural choices.

---

## License

MIT © Georgi Mullassery
