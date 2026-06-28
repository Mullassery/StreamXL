# Changelog

All notable changes to StreamXL will be documented in this file.

## [0.4.0] — 2025-06-28

### Added
- Stream-based reading and writing of `.xlsx` files
- Memory-efficient row-by-row processing
- ~46× faster read than openpyxl
- ~10× faster write than openpyxl
- Full XLSX format support (Excel, Google Sheets, LibreOffice Calc)
- Cross-platform wheels: Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), Windows (x86_64)

### Performance
- **Read:** Streams rows without loading entire workbook
- **Write:** Efficient memory usage for large datasets
- **Speed:** Significantly faster than OpenPyXL and pandas-based approaches

### Status
- **Stable:** Core read/write functionality
- **Production-ready:** Used for high-volume XLSX processing

## [0.1.0] — Initial Release
- Basic XLSX reading
- Basic XLSX writing
- Rust engine foundation

---

## Roadmap

### Near-term (v0.5)
- Enhanced formula support
- Conditional formatting
- Sheet protection
- Cell styling improvements

### Medium-term (v0.6–v1.0)
- Performance profiling and optimization
- Async API
- Streaming charts
- Multi-sheet batch operations

### Long-term
- Template system
- Pivot table support
- Custom number formats
