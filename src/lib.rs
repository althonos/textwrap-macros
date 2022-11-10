//! *Simple procedural macros to use [`textwrap`] utilities at compile time.*
//!
//! [`textwrap`]: https://github.com/mgeisler/textwrap
//!
//! [![TravisCI](https://img.shields.io/travis/com/althonos/textwrap-macros/master.svg?maxAge=600&style=flat-square)](https://travis-ci.com/althonos/textwrap-macros/branches)
//! [![Codecov](https://img.shields.io/codecov/c/gh/althonos/textwrap-macros/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/textwrap-macros)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/textwrap-macros)
//! [![Crate](https://img.shields.io/crates/v/textwrap-macros.svg?maxAge=600&style=flat-square)](https://crates.io/crates/textwrap-macros)
//! [![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/textwrap-macros)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/textwrap-macros.rs/blob/master/CHANGELOG.md)
//!
//! ## Usage
//!
//! Either use the macros using the old-style `#[macro_use]` or import them as
//! any other crate member:
//! ```rust
//! use textwrap_macros::dedent;
//!
//! const poem: &str = dedent!(r#"
//!       When we two parted
//!       In silence and tears,
//!       Half broken-hearted
//!       To sever for years,
//!       Pale grew thy cheek and cold,
//!       Colder thy kiss;
//!       Truly that hour foretold
//!       Sorrow to this.
//! "#);
//! ```
//!
//! Checkout the [documentation of the original library](https://docs.rs/textwrap/0.12.0/textwrap/)
//! for more information about the behaviour of each of the wrapped functions.
//!
//! ## Changelog
//!
//! This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
//! and provides a [changelog](https://github.com/althonos/textwrap-macros/blob/master/CHANGELOG.md)
//! in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro_hack;

#[proc_macro_hack::proc_macro_hack]
/// Removes common leading whitespace from each line.
///
/// This macro will look at each non-empty line and determine the maximum
/// amount of whitespace that can be removed from all lines, and create a
/// new string literal in place of the old one.
///
/// # Usage
/// ```rust,ignore
/// dedent!($text: lit &str) -> lit &str
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::dedent;
///
/// const X: &str = dedent!("
///   1st line
///     2nd line
///   3rd line
/// ");
///
/// assert_eq!(X, "
/// 1st line
///   2nd line
/// 3rd line
/// ");
/// ```
///
/// See also [`textwrap::dedent`](https://docs.rs/textwrap/latest/textwrap/fn.dedent.html).
pub use textwrap_macros_impl::dedent;

#[proc_macro_hack::proc_macro_hack]
/// Add prefix to each non-empty line.
///
/// Empty lines (consisting only of whitespaces, with respect to
/// [`char.is_whitespace`](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace))
/// are not indented but replaced by a single newline character `\n`. Leading
/// and trailing whitespace on non-empty lines is kept unchanged.
///
/// # Usage
/// ```rust,ignore
/// indent!($text: lit &str, $prefix: lit &str) -> lit &str
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::indent;
///
/// const Y: &str = indent!("\
/// Foo
/// Bar
/// ", "-> ");
///
/// assert_eq!(Y, "\
/// -> Foo
/// -> Bar
/// ");
/// ```
///
/// See also [textwrap::indent](https://docs.rs/textwrap/latest/textwrap/fn.indent.html).
pub use textwrap_macros_impl::indent;

#[proc_macro_hack::proc_macro_hack]
/// Fill a line of text at `width` characters.
///
/// Strings are wrapped based on their displayed width, not their size in bytes.
/// The result is a string with newlines between each line. Use `wrap` if
/// you need access to the individual lines.
///
/// # Usage
/// ```rust, ignore
/// fill!($text: lit &str, $width: lit usize) -> lit &str
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::fill;
///
/// const FILLED: &str = fill!("Memory safety without garbage collection.", 15);
/// assert_eq!(FILLED, "Memory safety\nwithout garbage\ncollection.");
/// ```
///
/// See also [textwrap::fill](https://docs.rs/textwrap/latest/textwrap/fn.fill.html).
pub use textwrap_macros_impl::fill;

#[proc_macro_hack::proc_macro_hack]
/// Wrap a line of text at width characters.
///
/// Strings are wrapped based on their displayed width, not their size in
/// bytes.
///
/// # Usage
/// ```rust, ignore
/// wrap!($text: lit &str, $width: lit usize) -> lit &[ lit &str ]
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::wrap;
///
/// const LINES: &[&str] = wrap!("Concurrency without data races.", 15);
/// assert_eq!(LINES, ["Concurrency", "without data", "races."]);
/// ```
pub use textwrap_macros_impl::wrap;

#[proc_macro_hack::proc_macro_hack]
/// Unpack a paragraph of already-wrapped text.
///
/// This function attempts to recover the original text from a single paragraph
/// of text. In addition, it will recognize a common prefix and a common line
/// ending among the lines.
///
/// # Usage
/// ```rust, ignore
/// unfill!($text: lit &str) -> lit &str
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::unfill;
///
/// const TEXT: &str = unfill!("\
///     Concurrency
///     without data
///     races.
/// ");
/// assert_eq!(TEXT, "Concurrency without data races.\n");
/// ```
pub use textwrap_macros_impl::unfill;

#[proc_macro_hack::proc_macro_hack]
/// Refill a paragraph of wrapped text with a new width.
///
/// This function will first use the `unfill` function to remove newlines from
/// the text. Afterwards the text is filled again using the `fill` function.
///
/// # Usage
/// ```rust, ignore
/// fill!($text: lit &str, $width: lit usize) -> lit &str
/// ```
///
/// # Example
/// ```rust
/// use textwrap_macros::refill;
///
/// const TEXT: &str = refill!("\
/// Concurrency
/// without data
/// races.
/// ", 20);
/// assert_eq!(TEXT, "\
/// Concurrency without
/// data races.
/// ");
/// ```
pub use textwrap_macros_impl::refill;
