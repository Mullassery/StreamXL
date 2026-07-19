"""OKF Sheet Metadata for PyStreamXL.

Spreadsheet operation tracking, formula library, and transformation history
for data streaming in spreadsheets.
"""

from pathlib import Path
from typing import Dict, List, Optional
import json
from datetime import datetime


class OKFSheetMetadata:
    """Track spreadsheet operations and transformations."""

    def __init__(self, metadata_dir: Path = None):
        self.metadata_dir = metadata_dir or Path.cwd() / "sheet_metadata"
        self.metadata_dir.mkdir(exist_ok=True)

    def record_operation(self, sheet_name: str, operation: str,
                        row_count: int, column_count: int) -> None:
        """Record spreadsheet operation."""
        entry = {
            'timestamp': datetime.now().isoformat(),
            'sheet': sheet_name,
            'operation': operation,
            'rows': row_count,
            'columns': column_count
        }

        log_file = self.metadata_dir / f"operations_{sheet_name}.jsonl"
        with open(log_file, 'a') as f:
            f.write(json.dumps(entry) + '\n')

    def get_operation_history(self, sheet_name: str) -> List[Dict]:
        """Get operation history for sheet."""
        log_file = self.metadata_dir / f"operations_{sheet_name}.jsonl"
        if not log_file.exists():
            return []

        operations = []
        with open(log_file) as f:
            for line in f:
                operations.append(json.loads(line))

        return operations

    def save_formula_library(self, sheet_name: str, formulas: Dict) -> None:
        """Save reusable formulas."""
        filename = f"formulas_{sheet_name}.json"
        with open(self.metadata_dir / filename, 'w') as f:
            json.dump(formulas, f, indent=2)
