"""Security utilities for StreamXL."""

from pathlib import Path
from typing import Union


def validate_xlsx_path(path: Union[str, Path]) -> Path:
    """
    Validate Excel file path.
    
    Args:
        path: File path provided by user
        
    Returns:
        Validated Path object
        
    Raises:
        ValueError: If path is invalid or dangerous
    """
    path = Path(path).resolve()
    
    # Ensure it's an Excel file
    if path.suffix.lower() not in ['.xlsx', '.xls']:
        raise ValueError(f"Must be Excel file (.xlsx or .xls), got: {path.suffix}")
    
    # Prevent directory traversal in filename
    if '..' in str(path):
        raise ValueError("Path traversal not allowed")
    
    return path


def validate_read_path(path: Union[str, Path]) -> Path:
    """Validate path for reading (must exist and be file)."""
    path = validate_xlsx_path(path)
    
    if not path.exists():
        raise ValueError(f"File not found: {path}")
    
    if not path.is_file():
        raise ValueError(f"Not a file: {path}")
    
    return path


def validate_write_path(path: Union[str, Path]) -> Path:
    """Validate path for writing (parent must exist)."""
    path = validate_xlsx_path(path)
    
    if not path.parent.exists():
        raise ValueError(f"Parent directory doesn't exist: {path.parent}")
    
    # Warn if file will be overwritten
    if path.exists():
        print(f"Warning: File will be overwritten: {path}")
    
    return path
