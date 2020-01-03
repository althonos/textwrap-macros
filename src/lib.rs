extern crate proc_macro_hack;

#[proc_macro_hack::proc_macro_hack]
/// Removes common leading whitespace from each line.
///
/// This macro will look at each non-empty line and determine the maximum
/// amount of whitespace that can be removed from all lines, and create a
/// new string literal in place of the old one.
///
/// ```rust
/// # extern crate textwrap_macros;
/// # use textwrap_macros::dedent;
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
pub use textwrap_macros_impl::dedent;

#[proc_macro_hack::proc_macro_hack]
/// Add prefix to each non-empty line.
pub use textwrap_macros_impl::indent;
