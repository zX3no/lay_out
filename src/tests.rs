use crate::*;

#[test]
fn flex_horizontal_new() {
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

    let mut flex = h!(h1, h2, h3).width(50).height(40);

    flex.force_draw();

    assert_eq!(flex.debug[0].0.x, 0);
    assert_eq!(flex.debug[0].0.y, 0);

    assert_eq!(flex.debug[1].0.x, 20);
    assert_eq!(flex.debug[1].0.y, 0);

    assert_eq!(flex.debug[2].0.x, 0);
    assert_eq!(flex.debug[2].0.y, 20);

    flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::TopRight);
    flex.force_draw();

    assert_eq!(flex.debug[0].0.x, 50);
    assert_eq!(flex.debug[0].0.y, 0);

    assert_eq!(flex.debug[1].0.x, 30);
    assert_eq!(flex.debug[0].0.y, 0);

    assert_eq!(flex.debug[2].0.x, 50);
    assert_eq!(flex.debug[2].0.y, 20);

    flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::BottomLeft);
    flex.force_draw();

    assert_eq!(flex.debug[0].0.x, 0);
    assert_eq!(flex.debug[0].0.y, 40);

    assert_eq!(flex.debug[1].0.x, 20);
    assert_eq!(flex.debug[0].0.y, 40);

    assert_eq!(flex.debug[2].0.x, 0);
    assert_eq!(flex.debug[2].0.y, 20);

    flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::BottomRight);
    flex.force_draw();

    assert_eq!(flex.debug[0].0.x, 50);
    assert_eq!(flex.debug[0].0.y, 40);

    assert_eq!(flex.debug[1].0.x, 30);
    assert_eq!(flex.debug[0].0.y, 40);

    assert_eq!(flex.debug[2].0.x, 50);
    assert_eq!(flex.debug[2].0.y, 20);
}

#[test]
fn flex_center_new() {
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

    let mut h4 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    {
        let mut flex = flex!(h1, h2)
            .mode(FlexMode::Center(Center::Horizontal))
            .wh(40);

        flex.force_draw();

        assert_eq!(flex.debug.len(), 2);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);
    }

    {
        let mut flex = flex!(h1, h2, h3)
            .mode(FlexMode::Center(Center::Horizontal))
            .wh(40);

        flex.force_draw();

        assert_eq!(flex.debug.len(), 3);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);
        //Middle is (40 / 2) - ((40 / 2) / 2) = 10
        assert_eq!(flex.debug[2].0.x, 10);
    }

    {
        let mut flex = flex!(h1, h2, h3, h4)
            .mode(FlexMode::Center(Center::Horizontal))
            .wh(40);

        flex.force_draw();

        assert_eq!(flex.debug.len(), 4);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);
        assert_eq!(flex.debug[2].0.x, 0);
        assert_eq!(flex.debug[3].0.x, 20);
    }
}

#[test]
fn flex_horizontal() {
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
    let vw = 50;
    let vh = 40;

    let test = flex_standard!(Quadrant::TopLeft, Direction::Horizontal, vw, vh, h1, h2, h3);

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 20);
    assert_eq!(test[1].0.y, 0);

    assert_eq!(test[2].0.x, 0);
    assert_eq!(test[2].0.y, 20);

    let test = flex_standard!(
        Quadrant::TopRight,
        Direction::Horizontal,
        vw,
        vh,
        h1,
        h2,
        h3
    );
    assert_eq!(test[0].0.x, 50);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 30);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[2].0.x, 50);
    assert_eq!(test[2].0.y, 20);

    let test = flex_standard!(
        Quadrant::BottomLeft,
        Direction::Horizontal,
        vw,
        vh,
        h1,
        h2,
        h3
    );

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 40);

    assert_eq!(test[1].0.x, 20);
    assert_eq!(test[0].0.y, 40);

    assert_eq!(test[2].0.x, 0);
    assert_eq!(test[2].0.y, 20);

    let test = flex_standard!(
        Quadrant::BottomRight,
        Direction::Horizontal,
        vw,
        vh,
        h1,
        h2,
        h3
    );

    assert_eq!(test[0].0.x, 50);
    assert_eq!(test[0].0.y, 40);

    assert_eq!(test[1].0.x, 30);
    assert_eq!(test[0].0.y, 40);

    assert_eq!(test[2].0.x, 50);
    assert_eq!(test[2].0.y, 20);

    let vw = 70;
    let vh = 40;

    let test = flex_center!(Center::Horizontal, vw, vh, h1, h2);
    assert_eq!(test.len(), 2);

    assert_eq!(test[0].0.x, 10);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 40);
    assert_eq!(test[1].0.y, 0);
}

#[test]
fn flex_vertical() {
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
    let vw = 50;
    let vh = 50;

    let test = flex_standard!(Quadrant::TopLeft, Direction::Vertical, vw, vh, h1, h2, h3);

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 0);
    assert_eq!(test[1].0.y, 20);

    assert_eq!(test[2].0.x, 20);
    assert_eq!(test[2].0.y, 0);

    let test = flex_standard!(Quadrant::TopRight, Direction::Vertical, vw, vh, h1, h2, h3);

    assert_eq!(test[0].0.x, 50);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 50);
    assert_eq!(test[1].0.y, 20);

    assert_eq!(test[2].0.x, 30);
    assert_eq!(test[2].0.y, 0);

    let test = flex_standard!(
        Quadrant::BottomLeft,
        Direction::Vertical,
        vw,
        vh,
        h1,
        h2,
        h3
    );

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 50);

    assert_eq!(test[1].0.x, 0);
    assert_eq!(test[1].0.y, 30);

    assert_eq!(test[2].0.x, 20);
    assert_eq!(test[2].0.y, 50);

    let test = flex_standard!(
        Quadrant::BottomRight,
        Direction::Vertical,
        vw,
        vh,
        h1,
        h2,
        h3
    );

    assert_eq!(test[0].0.x, 50);
    assert_eq!(test[0].0.y, 50);

    assert_eq!(test[1].0.x, 50);
    assert_eq!(test[1].0.y, 30);

    assert_eq!(test[2].0.x, 30);
    assert_eq!(test[2].0.y, 50);
}

#[test]
fn hcenter() {
    let vw = 40;
    let vh = 40;

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

    let test = flex_center!(Center::Horizontal, vw, vh, h1, h2);
    assert_eq!(test.len(), 2);
    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[1].0.x, 20);

    let mut header3 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    let test = flex_center!(Center::Horizontal, vw, vh, h1, h2, header3);
    assert_eq!(test.len(), 3);
    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[1].0.x, 20);
    //Middle is (40 / 2) - ((40 / 2) / 2) = 10
    assert_eq!(test[2].0.x, 10);

    let mut header4 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };

    let test = flex_center!(Center::Horizontal, vw, vh, h1, h2, header3, header4);

    assert_eq!(test.len(), 4);
    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[1].0.x, 20);
    assert_eq!(test[2].0.x, 0);
    assert_eq!(test[3].0.x, 20);
}

#[test]
fn vcenter() {
    let vw = 40;
    let vh = 40;

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

    let test = flex_center!(Center::Vertical, vw, vh, h1, h2, h3);
    assert_eq!(test.len(), 3);

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 0);

    assert_eq!(test[1].0.x, 0);
    assert_eq!(test[1].0.y, 20);

    assert_eq!(test[2].0.x, 20);
    assert_eq!(test[2].0.y, 10);

    let vw = 40;
    let vh = 70;

    let test = flex_center!(Center::Vertical, vw, vh, h1, h2);
    assert_eq!(test.len(), 2);

    assert_eq!(test[0].0.x, 0);
    assert_eq!(test[0].0.y, 10);

    assert_eq!(test[1].0.x, 0);
    assert_eq!(test[1].0.y, 40);
}
