use lay_out::*;

fn main() {
    let mut h = Header {
        title: "hi",
        area: Rect { x: 0, y: 0, width: 20, height: 20 },
    };

    let mut h2 = Header {
        title: "hi",
        area: Rect { x: 0, y: 0, width: 20, height: 20 },
    };

    let mut h3 = Header {
        title: "hi",
        area: Rect { x: 0, y: 0, width: 20, height: 20 },
    };

    unsafe {
        VIEWPORT_HEIGHT = 20;
        VIEWPORT_WIDTH = 50;
    }

    let test = flex!(Flex::TopLeft, h, h2, h3);

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 20);
    assert_eq!(test[1].0.y, 0);

    assert_eq!(test[2].0.x, 0);
    assert_eq!(test[2].0.y, 20);

    let test = flex!(Flex::TopRight, h, h2, h3);
    assert_eq!(test[0].0.x, 50);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 50 - 20);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[2].0.x, 50);
    assert_eq!(test[2].0.y, 20);
}
