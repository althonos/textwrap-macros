#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate textwrap_macros;

#[test]
fn test_constant() {
    const X: &[&str] = wrap!("Concurrency without data races.", 15);
    const Y: &[&str] = &["Concurrency", "without data", "races."];
    assert_eq!(X, Y);
}

#[test]
fn test_static() {
    static X: &[&str] = wrap!("Concurrency without data races.", 15);
    static Y: &[&str] = &["Concurrency", "without data", "races."];
    assert_eq!(X, Y);
}
