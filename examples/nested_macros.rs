#![allow(static_mut_refs, unused)]
use lay_out::*;

struct Closure<T: Fn(Rect) -> Rect>(T);

impl<T: Fn(Rect) -> Rect> Closure<T> {
    pub fn closure(self) -> T {
        self.0
    }
}

macro_rules! v {
    ($($w:expr),*) => {{
        |mut area: Rect| {
            let count = const { count_expr!($($w),*) };
            let height = area.height / count;

            $(
                let _w = $w;
                area.y += height;
            )*

            area
        }
    }};
}

macro_rules! h {
    ($($w:expr),*) => {{
        Closure(|mut area: Rect| {
            let count = const { count_expr!($($w),*) };
            let width = area.width / count;

            $(
                let _w = $w.closure();
                _w(area);

                area.x += width;
            )*

            area
        })
    }};
}

macro_rules! root {
    ($($w:expr),*) => {{
        let mut area = Rect::new(0, 0, 20, 20);

            $(
                area = $w(area);
            )*
            area

    }};
}

fn main() {
    let flex = root!(v!(rect(), rect()));
    dbg!(flex);

    let mut h1 = Header {
        title: "hi",
        area: Rect {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        },
    };
    flex!(h1, header());

    // root.closure()(Rect::new(0, 0, 20, 20));
    // h!(rect(), v!(rect()));

    // h!(rect(), v!(rect(), h!(rect(), rect())));
}
