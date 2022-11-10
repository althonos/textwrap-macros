extern crate proc_macro;
extern crate syn;
extern crate textwrap;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::parse_macro_input;

// ---------------------------------------------------------------------------

struct DedentInput {
    lit: syn::LitStr,
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

    syn::LitStr::new(&newstr, input.lit.span())
        .into_token_stream()
        .into()
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
    let width: usize = input.width.base10_parse().expect("could not parse number");
    let newstr = textwrap::fill(&input.lit.value(), width);

    syn::LitStr::new(&newstr, input.lit.span())
        .into_token_stream()
        .into()
}

// ---------------------------------------------------------------------------

struct RefillInput {
    lit: syn::LitStr,
    width: syn::LitInt,
}

impl Parse for RefillInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let width = input.parse()?;
        Ok(Self { lit, width })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn refill(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as RefillInput);
    let width: usize = input.width.base10_parse().expect("could not parse number");
    let newstr = textwrap::refill(&input.lit.value(), width);

    syn::LitStr::new(&newstr, input.lit.span())
        .into_token_stream()
        .into()
}

// ---------------------------------------------------------------------------

struct UnfillInput {
    lit: syn::LitStr,
}

impl Parse for UnfillInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            lit: input.parse()?,
        })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn unfill(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as UnfillInput);
    let newstr = textwrap::unfill(&input.lit.value()).0;

    syn::LitStr::new(&newstr, input.lit.span())
        .into_token_stream()
        .into()
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

    syn::LitStr::new(&newstr, input.lit.span())
        .into_token_stream()
        .into()
}

// ---------------------------------------------------------------------------

struct WrapInput {
    lit: syn::LitStr,
    width: syn::LitInt,
}

impl Parse for WrapInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let width = input.parse()?;
        Ok(Self { lit, width })
    }
}

#[proc_macro_hack::proc_macro_hack]
pub fn wrap(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as WrapInput);
    let width: usize = input.width.base10_parse().expect("could not parse number");

    let elems = textwrap::wrap(&input.lit.value(), width)
        .iter()
        .map(|s| syn::Lit::from(syn::LitStr::new(&s, input.lit.span())))
        .map(|lit| {
            syn::Expr::Lit(syn::ExprLit {
                lit,
                attrs: Vec::new(),
            })
        })
        .collect();
    let array = syn::ExprArray {
        elems,
        attrs: Vec::new(),
        bracket_token: Default::default(),
    };
    let expr = syn::ExprReference {
        attrs: Vec::new(),
        and_token: Default::default(),
        raw: Default::default(),
        mutability: None,
        expr: Box::new(syn::Expr::Array(array)),
    };

    expr.into_token_stream().into()
}
