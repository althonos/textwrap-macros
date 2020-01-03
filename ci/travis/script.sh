#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test with coverage -----------------------------------------------------

if cargo tarpaulin -V >/dev/null 2>&1; then
	log Measuring code coverage with Tarpaulin
	cargo tarpaulin -v --out Xml --ciserver travis-ci
else
	log Testing code without coverage
	cargo test
fi
