# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/althonos/textwrap-macros/compare/v0.2.4...HEAD

## [v0.2.4] - 2021-01-04

### Changed
- Bumped `textwrap` dependency to `v0.13.0` to use new wrapping algorithm
  ([#5](https://github.com/althonos/textwrap-macros/pull/5)).

[v0.2.4]: https://github.com/althonos/textwrap-macros/compare/v0.2.3...v0.2.4


## [v0.2.3] - 2020-06-29

### Changed
- Bumped `textwrap` dependency to `v0.12.0` to support ANSI escape codes
  ([#1](https://github.com/althonos/textwrap-macros/pull/1)).

[v0.2.3]: https://github.com/althonos/textwrap-macros/compare/v0.2.2...v0.2.3


## [v0.2.2] - 2020-04-10

### Fixed
- Potential build issues with `1.0.17` version of `syn` because of missing
  features in `textwrap-macros-impl`.

[v0.2.2]: https://github.com/althonos/textwrap-macros/compare/v0.2.1...v0.2.2


## [v0.2.1] - 2020-01-23

### Fixed
- Travis-CI build button not rendering properly on `README.md`.

### Changed
- Implementation does convert token stream directly from `syn` types instead of
  parsing intermediate Rust code.

[v0.2.1]: https://github.com/althonos/textwrap-macros/compare/v0.2.0...v0.2.1


## [v0.2.0] - 2020-01-15

### Added
- Implementation of the [`textwrap::wrap`] function as a macro producing a
  inline static slice of static strings.

[v0.2.0]: https://github.com/althonos/textwrap-macros/compare/v0.1.0...v0.2.0
[`textwrap::wrap`]: https://docs.rs/textwrap/0.11.0/textwrap/fn.wrap.html


## [v0.1.0] - 2020-01-07

Initial release.

[v0.1.0]: https://github.com/althonos/textwrap-macros/compare/c55dc29...v0.1.0
