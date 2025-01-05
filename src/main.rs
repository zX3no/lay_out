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
    let vw = 100;
    let vh = 40;
    // let test = flex_center!(Center::Both, vw, vh, h1);

    // assert_eq!(test.len(), 1);
    // assert_eq!(test[0].0.x, 10);
    // assert_eq!(test[0].0.y, 10);

    let test = flex_center!(Center::Horizontal, vw, vh, h1, h2, h3);

    let mut flex = v!(h1, h2, h3).wh(40);
    flex.force_draw();
    dbg!(flex);

    // dbg!(&test);

    // assert_eq!(test.len(), 3);

    // assert_eq!(test[0].0.x, 10);
    // assert_eq!(test[0].0.y, 10);

    // assert_eq!(test[1].0.x, 40);
    // assert_eq!(test[1].0.y, 10);

    // assert_eq!(test[2].0.x, 70);
    // assert_eq!(test[2].0.y, 10);
}
