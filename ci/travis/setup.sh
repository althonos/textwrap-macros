#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Setup cargo-tarpaulin ----------------------------------------------------------

LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
LOCAL=$(cargo tarpaulin --version 2>/dev/null | cut -d" " -f3 || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
	log Installing cargo-tarpaulin v$LATEST
	cargo install -f cargo-tarpaulin --root "$HOME/.cargo"
else
	log Using cached cargo-tarpaulin v$LOCAL
fi


# --- Setup cargo-cache ------------------------------------------------------

LATEST=$(cargo search cargo-cache | head -n1 | cut -f2 -d"\"")
LOCAL=$(cargo cache --version 2>/dev/null | cut -d" " -f2 || echo "none")

if [ "$LATEST" != "$LOCAL" ]; then
        log Installing cargo-cache v$LATEST
        cargo install -f cargo-cache --root "$HOME/.cargo"
else
        log Using cached cargo-cache v$LOCAL
fi
