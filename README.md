# `textwrap-macros`

*Simple procedural macros to use [`textwrap`] utilities at compile time.*

[`textwrap`]: https://github.com/mgeisler/textwrap

[![TravisCI](https://img.shields.io/travis/althonos/textwrap-macros/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/althonos/textwrap-macros/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/textwrap-macros/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/textwrap-macros)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/textwrap-macros)
[![Crate](https://img.shields.io/crates/v/textwrap-macros.svg?maxAge=600&style=flat-square)](https://crates.io/crates/textwrap-macros)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/textwrap-macros)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/textwrap-macros.rs/blob/master/CHANGELOG.md)


## Usage

Add the `textwrap-macros` crate to the `Cargo.toml` manifest:

```toml
[dependencies]
textwrap-macros = "0.1"
```

Then either use the macros using the old-style `#[macro_use]` or import them as
any other crate member:
```rust
#[macro_use]
extern crate textwrap_macros;

const poem: &str = dedent!(r#"
      When we two parted
      In silence and tears,
      Half broken-hearted
      To sever for years,
      Pale grew thy cheek and cold,
      Colder thy kiss;
      Truly that hour foretold
      Sorrow to this.
"#);
```

Checkout the [documentation of the original library](https://docs.rs/textwrap/)
for more information about the behaviour of each of the wrapped functions.

## Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/textwrap-macros/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.
