#!/usr/bin/env bash
set -euo pipefail

echo "Building streamxl..."
maturin develop --release
echo "Build complete."
