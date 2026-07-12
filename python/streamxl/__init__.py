from .api import read, stream, write, writer, sheets, read_all, append
from .core import XlsxWriter

# CRITICAL: Formula preservation support (unblocks finance teams)
from ._formula_support import (
    FormulaAnalyzer,
    FormulaPreserver,
    FormulaSubstitution,
    FormulaType,
    FormulaCell,
    FormulaMapping,
)

__all__ = [
    "read", "stream", "write", "writer", "sheets", "read_all", "append", "XlsxWriter",
    # Formula support (v1.2.0+)
    "FormulaAnalyzer",
    "FormulaPreserver",
    "FormulaSubstitution",
    "FormulaType",
    "FormulaCell",
    "FormulaMapping",
]
__version__ = "0.4.0"
