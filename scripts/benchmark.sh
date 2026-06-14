#!/usr/bin/env bash
set -euo pipefail

FILE="${1:-benchmarks/large_file_test.xlsx}"

if [ ! -f "$FILE" ]; then
  echo "No benchmark file at $FILE. Generate one or provide a path."
  exit 1
fi

echo "Running benchmark on $FILE..."
python benchmarks/openpyxl_vs_streamxl.py "$FILE"
