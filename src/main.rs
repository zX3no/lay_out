#![allow(unused)]
use lay_out::*;

fn main() {
    // grow!(rect(), rect(), v2!(rect(), rect()))

    //No color because it's just a quick mockup.
    //Skip the post-fix builder functions for now, makes the code simple.
    let mut blue = rect().width(960).height(540);

    let mut pink = rect().width(300).height(300);
    let mut yellow = rect().width(350).height(200);

    // grow!(pink, yellow);
    // assert!(pink.x == 32);
    // assert!(yellow.x == pink.x + pink.width + 32);

    let flex = flex!(h!(rect().wh(300), rect().w(300).h(200)).gap(32))
        .padding(32)
        .gap(32);

    let area = flex.call();
    dbg!(area);

    // let direction =
    // let padding = $crate::Padding::new(32, 32, 32, 32);
    // let gap = 32;

    let defer = h!(rect().wh(300), rect().w(300).h(200)).gap(32);
    let container = defer.call();
    assert!(container.area.width == 632);
    assert!(container.area.height == 300);
    assert!(container.widgets.len() == 2)
}
