extern crate proc_macro_hack;

#[proc_macro_hack::proc_macro_hack]
pub use textwrap_macros_impl::dedent;

#[proc_macro_hack::proc_macro_hack]
pub use textwrap_macros_impl::indent;
