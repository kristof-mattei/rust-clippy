#![warn(clippy::pattern_type_mismatch)]
#![allow(
    clippy::match_ref_pats,
    clippy::never_loop,
    clippy::redundant_pattern_matching,
    clippy::single_match
)]

//@aux-build:external.rs
use external::macro_with_match;

fn main() {}

fn external_macro_expansion() {
    macro_with_match! {
        ()
    };
}
