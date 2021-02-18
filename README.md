# `textwrap-macros` [![Star me](https://img.shields.io/github/stars/althonos/textwrap-macros.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/textwrap-macros/stargazers)

*Simple procedural macros to use [`textwrap`] utilities at compile time.*

[`textwrap`]: https://github.com/mgeisler/textwrap

[![Actions](https://img.shields.io/github/workflow/status/althonos/textwrap-macros/Test?style=flat-square&maxAge=600)](https://github.com/althonos/textwrap-macros/actions)
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
textwrap-macros = "0.2"
```

Then either use the macros using the old-style `#[macro_use]` or import them as
any other crate member:
```rust
use textwrap_macros::dedent;

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

Macros usage with small examples can be found on
[`docs.rs`](https://docs.rs/textwrap-macros). The following functions have been
ported into macros:

- [x] [`dedent`](https://docs.rs/textwrap-macros/latest/textwrap_macros/macro.dedent.html)
- [x] [`fill`](https://docs.rs/textwrap-macros/latest/textwrap_macros/macro.fill.html)
- [x] [`indent`](https://docs.rs/textwrap-macros/latest/textwrap_macros/macro.indent.html)
- [x] [`wrap`](https://docs.rs/textwrap-macros/latest/textwrap_macros/macro.wrap.html)

Check out the [documentation of the original library](https://docs.rs/textwrap/)
for more information about the behaviour of each of the wrapped functions.


## Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/textwrap-macros/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.
