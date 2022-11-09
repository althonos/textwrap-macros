#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate textwrap_macros;

#[test]
fn test_constant() {
    const X: &str = indent!("abc\ndef\nghi\njkl\n", "- ");
    const Y: &str = "- abc\n- def\n- ghi\n- jkl\n";
    assert_eq!(X, Y);
}

#[test]
fn test_static() {
    static X: &str = indent!("abc\ndef\nghi\njkl\n", "- ");
    static Y: &str = "- abc\n- def\n- ghi\n- jkl\n";
    assert_eq!(X, Y);
}
