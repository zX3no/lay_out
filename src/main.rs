#![allow(static_mut_refs)]
pub use lay_out::*;

fn main() {
    let mut h1 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    let mut h2 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    let mut h3 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    //viewport width and height.

    let mut flex = f!(h1, h2, h3);
    flex.viewport_width = 50;
    flex.viewport_height = 40;
    // flex.flex = FlexMode::Center(Center::Vertical);
    flex.force_draw();

    assert_eq!(flex.debug[0].0.x, 0);
    assert_eq!(flex.debug[0].0.y, 0);

    assert_eq!(flex.debug[1].0.x, 20);
    assert_eq!(flex.debug[1].0.y, 0);

    assert_eq!(flex.debug[2].0.x, 0);
    assert_eq!(flex.debug[2].0.y, 20);
}
