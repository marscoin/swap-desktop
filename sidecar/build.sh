#!/usr/bin/env bash
# Build the engine (atomicswapd + Electrum-Mars) as one binary for Tauri.
#
#   sidecar/build.sh <path-to-libsecp256k1>
#
# Expects atomicswapd (with its dashboard built) and Electrum-Mars to be
# pip-installed in the current Python. Writes
# src-tauri/binaries/atomicswapd-<target-triple>[.exe], the name Tauri's
# externalBin needs.
set -euo pipefail
cd "$(dirname "$0")/.."
SECP="${1:?path to the libsecp256k1 shared library}"
TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"
EXT=""; [[ "$TRIPLE" == *windows* ]] && EXT=".exe"
case "$TRIPLE" in
  *apple*)   SECP_NAME="libsecp256k1.0.dylib" ;;
  *windows*) SECP_NAME="libsecp256k1-0.dll" ;;
  *)         SECP_NAME="libsecp256k1.so.0" ;;
esac
SEP=":"; [[ "$TRIPLE" == *windows* ]] && SEP=";"
# Absolute, native paths: PyInstaller reads relative ones against the spec
# folder, and Windows Python can't use Git Bash's /d/... form.
ROOT="$(pwd -W 2>/dev/null || pwd)"

cp "$SECP" "sidecar/$SECP_NAME"
pyinstaller --noconfirm --clean --onefile --name atomicswapd \
  --distpath sidecar/dist --workpath sidecar/build --specpath sidecar \
  --collect-submodules electrum_mars.plugins.atomic_swap \
  --collect-data electrum_mars \
  --collect-data atomicswapd \
  --hidden-import _scrypt \
  --add-binary "$ROOT/sidecar/$SECP_NAME${SEP}electrum_mars" \
  "$ROOT/sidecar/entry.py"

mkdir -p src-tauri/binaries
cp "sidecar/dist/atomicswapd$EXT" "src-tauri/binaries/atomicswapd-$TRIPLE$EXT"
echo "built src-tauri/binaries/atomicswapd-$TRIPLE$EXT"
