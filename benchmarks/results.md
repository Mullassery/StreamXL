# Benchmark Results

**Environment:** Apple M-series, 16 GB RAM, Python 3.13, Rust 1.96, macOS

## 50,000 rows × 5 columns

| Library | Time | Peak Memory |
|---------|------|-------------|
| openpyxl (full load) | 4.30s | ~85 MB |
| openpyxl (read_only) | 3.09s | 4.4 MB |
| **streamxl** | **0.88s** | **10.7 MB** |

## 100,000 rows × 5 columns

| Library | Time | Peak Memory |
|---------|------|-------------|
| openpyxl (full load) | 8.70s | 188 MB |
| openpyxl (read_only) | 6.19s | 8.1 MB |
| **streamxl** | **1.79s** | **21 MB** |

**3.5× faster than openpyxl read_only. ~5× faster than full load.**

## Reproduce

```bash
# Generate a test file
python -c "
import openpyxl
wb = openpyxl.Workbook()
ws = wb.active
for i in range(1, 100001):
    ws.append([i, i*2, i*3, f'row_{i}', i*1.5])
wb.save('bench_100k.xlsx')
"

python benchmarks/openpyxl_vs_streamxl.py bench_100k.xlsx
```
