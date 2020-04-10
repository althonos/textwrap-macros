#!/bin/sh

set -e

. $(dirname $0)/functions.sh

# --- Test with coverage -----------------------------------------------------

log Measuring code coverage with Tarpaulin
cargo tarpaulin -v --out Xml --ciserver travis-ci

