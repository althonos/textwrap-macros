#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate textwrap_macros;

#[test]
fn test_constant() {
    const X: &str = dedent!("\t\tabc\n\tdef\n\t");
    const Y: &str = "\tabc\ndef\n";
    assert_eq!(X, Y);
}

#[test]
fn test_static() {
    static X: &str = dedent!("\t\tabc\n\tdef\n\t");
    static Y: &str = "\tabc\ndef\n";
    assert_eq!(X, Y);
}
