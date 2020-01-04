extern crate proc_macro;
extern crate syn;
extern crate textwrap;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;

// ---------------------------------------------------------------------------

struct DedentInput {
    lit: syn::LitStr
}

impl Parse for DedentInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            lit: input.parse()?,
        })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn dedent(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DedentInput);
    let newstr = textwrap::dedent(&input.lit.value());
    format!("{:?}", newstr).parse().unwrap()
}

// ---------------------------------------------------------------------------

struct FillInput {
    lit: syn::LitStr,
    width: syn::LitInt,
}

impl Parse for FillInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let width = input.parse()?;
        Ok(Self { lit, width })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn fill(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as FillInput);
    let width = input.width.base10_parse().expect("could not parse number");
    let newstr = textwrap::fill(&input.lit.value(), width);
    format!("{:?}", newstr).parse().unwrap()
}

// ---------------------------------------------------------------------------

struct IndentInput {
    lit: syn::LitStr,
    prefix: syn::LitStr,
}

impl Parse for IndentInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let prefix = input.parse()?;
        Ok(Self { lit, prefix })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn indent(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as IndentInput);
    let newstr = textwrap::indent(&input.lit.value(), &input.prefix.value());
    format!("{:?}", newstr).parse().unwrap()
}
