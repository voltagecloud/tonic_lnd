#!/usr/bin/env bash
set -euo pipefail

# Usage: ./bin/fetch_lnd_protos.sh <git-tag>
# Downloads LND proto files at the given tag and places them in vendor/ matching the LND tree.

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <git-tag>" >&2
  exit 1
fi

LND_TAG="$1"
LND_REPO="https://github.com/lightningnetwork/lnd.git"
TMP_DIR="$(mktemp -d)"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)"
VENDOR_DIR="$SCRIPT_DIR/../vendor"

# Required proto subdirs in LND repo
PROTO_PATHS=(
  "invoicesrpc/invoices.proto"
  "lightning.proto"
  "peersrpc/peers.proto"
  "routerrpc/router.proto"
  "signrpc/signer.proto"
  "verrpc/verrpc.proto"
  "walletrpc/walletkit.proto"
)

# Clean up temp dir on exit
# trap 'rm -rf "$TMP_DIR"' EXIT

# Clone the repo at the specified tag
GIT_TERMINAL_PROMPT=0 git clone --depth 1 --branch "$LND_TAG" "$LND_REPO" "$TMP_DIR/lnd"

cd "$TMP_DIR/lnd"

# Copy each proto file to vendor/ preserving directory structure
for relpath in "${PROTO_PATHS[@]}"; do
  src="$TMP_DIR/lnd/lnrpc/$relpath"
  dest="$VENDOR_DIR/$relpath"
  mkdir -p "$(dirname "$dest")"
  cp "$src" "$dest"
  echo "Copied $relpath"
done

echo "All proto files copied to $VENDOR_DIR."
