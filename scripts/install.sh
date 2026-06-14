#!/usr/bin/env sh
# streamxl installer
# Usage: curl -sSf https://raw.githubusercontent.com/Mullassery/StreamXL/main/scripts/install.sh | sh

set -e

REPO="https://github.com/Mullassery/StreamXL"
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
RESET='\033[0m'

say()  { printf "${CYAN}[streamxl]${RESET} %s\n" "$1"; }
ok()   { printf "${GREEN}[streamxl]${RESET} %s\n" "$1"; }
warn() { printf "${YELLOW}[streamxl]${RESET} %s\n" "$1"; }
die()  { printf "${RED}[streamxl] error:${RESET} %s\n" "$1" >&2; exit 1; }

say "Installing streamxl — Rust-powered streaming XLSX reader for Python"
echo ""

# ── 1. Check Python ────────────────────────────────────────────────────────────
if command -v python3 >/dev/null 2>&1; then
    PYTHON=python3
elif command -v python >/dev/null 2>&1; then
    PYTHON=python
else
    die "Python not found. Install Python 3.9+ from https://python.org and re-run."
fi

PY_VERSION=$("$PYTHON" -c "import sys; print('%d.%d' % sys.version_info[:2])")
PY_MAJOR=$("$PYTHON" -c "import sys; print(sys.version_info[0])")
PY_MINOR=$("$PYTHON" -c "import sys; print(sys.version_info[1])")

if [ "$PY_MAJOR" -lt 3 ] || { [ "$PY_MAJOR" -eq 3 ] && [ "$PY_MINOR" -lt 9 ]; }; then
    die "Python 3.9+ required (found $PY_VERSION). Please upgrade Python."
fi
say "Python $PY_VERSION found at $(command -v "$PYTHON")"

# ── 2. Choose package manager: uv > pip ───────────────────────────────────────
if command -v uv >/dev/null 2>&1; then
    PM="uv"
    say "Package manager: uv $(uv --version 2>/dev/null | head -1 | awk '{print $2}')"
elif "$PYTHON" -m pip --version >/dev/null 2>&1; then
    PM="pip"
    say "Package manager: pip"
else
    die "Neither uv nor pip found. Install pip: $PYTHON -m ensurepip"
fi

# ── 3. Try PyPI first (pre-built wheel — no Rust needed) ──────────────────────
say "Trying pre-built wheel from PyPI..."
if [ "$PM" = "uv" ]; then
    if uv pip install streamxl 2>/dev/null; then
        ok "Installed streamxl from PyPI via uv."
        install_ok=1
    fi
else
    if "$PYTHON" -m pip install streamxl --quiet 2>/dev/null; then
        ok "Installed streamxl from PyPI via pip."
        install_ok=1
    fi
fi

# ── 4. Fallback: build from source (requires Rust + maturin) ──────────────────
if [ -z "${install_ok}" ]; then
    warn "PyPI wheel not available — building from source."
    echo ""

    # Check Rust
    if ! command -v cargo >/dev/null 2>&1; then
        say "Rust not found. Installing via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet
        # shellcheck disable=SC1090
        . "$HOME/.cargo/env"
    fi
    say "Rust $(rustc --version | awk '{print $2}') found"

    # Check maturin
    if ! command -v maturin >/dev/null 2>&1; then
        say "Installing maturin build tool..."
        if [ "$PM" = "uv" ]; then
            uv pip install maturin --quiet
        else
            "$PYTHON" -m pip install maturin --quiet
        fi
    fi
    say "maturin $(maturin --version | awk '{print $2}') found"

    # Build from GitHub
    say "Building streamxl from $REPO ..."
    if [ "$PM" = "uv" ]; then
        uv pip install "git+${REPO}.git"
    else
        "$PYTHON" -m pip install "git+${REPO}.git" --quiet
    fi
    ok "Built and installed streamxl from source."
fi

# ── 5. Verify ──────────────────────────────────────────────────────────────────
echo ""
say "Verifying installation..."
if "$PYTHON" -c "import streamxl; print('streamxl', streamxl.__version__, 'ready')" 2>/dev/null; then
    echo ""
    ok "Installation complete!"
    echo ""
    echo "  Quick start:"
    echo "    import streamxl"
    echo "    for row in streamxl.read('your_file.xlsx'):"
    echo "        print(row)"
    echo ""
    echo "  Docs: $REPO"
else
    die "Installation verification failed. Try manually: $PYTHON -m pip install streamxl"
fi
