#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Publish crates to `crates.io` ------------------------------------------

log Deploying implementation crate
cargo publish --manifest-path impl/Cargo.toml --token $CRATES_IO_TOKEN

sleep 10
cargo update

log Deploying declaration crate
cargo publish --token $CRATES_IO_TOKEN


# --- Update release tags using Chandler -------------------------------------

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log Installing chandler gem
gem install --user-install chandler

log Updating GitHub release notes
chandler push --github="$TRAVIS_REPO_SLUG" --changelog="CHANGELOG.md"
