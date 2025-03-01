#![allow(static_mut_refs)]
// pub use lay_out::*;
use lay_out::count_expr;
use lay_out::Rect;

macro_rules! v {
    ($($w:expr),*) => {{
        //These will need to be passed in via a closure.
        let mut area = Rect::new(10, 0, 20, 10);

        let nexpr = const { count_expr!($($w),*) };
        let height = area.height / nexpr;

        $(
            let _w = $w;
            area.y += height;
        )*
    }};
}

macro_rules! h {
    ($($w:expr),*) => {{
        let mut area = Rect::new(0, 0, 20, 10);

        let nexpr = const { count_expr!($($w),*) };
        let width = area.width / nexpr;

        $(
            let _w = $w;
            area.x += width;
        )*

        dbg!(area);
    }};
}

fn rect() -> Rect {
    Rect::default()
}

fn main() {
    h!(rect(), v!(rect(), rect(), rect(), rect()));

    // h!(rect(), v!(rect(), h!(rect(), rect())));
}
