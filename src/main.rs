#![allow(static_mut_refs)]
pub use lay_out::*;

fn main() {
    let mut h = header();
    let _flex = flex!(h, header().x(0));
}
