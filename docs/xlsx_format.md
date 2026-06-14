# XLSX Format Notes

XLSX files are ZIP archives containing XML files:

```
workbook.zip/
├── xl/
│   ├── workbook.xml          # sheet index
│   ├── sharedStrings.xml     # string table
│   ├── styles.xml            # cell formatting
│   └── worksheets/
│       ├── sheet1.xml
│       └── sheet2.xml
├── [Content_Types].xml
└── _rels/
```

## sharedStrings

String cell values are stored as integer indices into `xl/sharedStrings.xml` to avoid repeating identical strings. A cell `<c t="s"><v>42</v></c>` means "look up index 42 in the SST."

## Cell types (`t` attribute)

| `t` value | Meaning |
|-----------|---------|
| `s`       | Shared string index |
| `b`       | Boolean (`1`/`0`) |
| `e`       | Error |
| *(absent)*| Numeric or date |
