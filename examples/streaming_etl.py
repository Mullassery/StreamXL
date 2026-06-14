"""Stream a large XLSX file and write to CSV — constant memory."""
import csv
import streamxl

INPUT = "large.xlsx"
OUTPUT = "output.csv"

with open(OUTPUT, "w", newline="") as f:
    writer = csv.writer(f)
    for row in streamxl.read(INPUT):
        writer.writerow(row)

print(f"Done. Written to {OUTPUT}")
