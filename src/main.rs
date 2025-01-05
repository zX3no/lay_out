#![allow(static_mut_refs)]
pub use lay_out::*;

fn main() {
    let mut h = header();
    let flex = flex!(h, header().x(0));

    // let flex = tflex!(h, header());

    // let w1 = &mut h;
    // let w2 = &mut header();

    // dbg!(w1, w2.as_uniform_layout_type());
}
