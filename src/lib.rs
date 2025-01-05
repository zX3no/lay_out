#![feature(associated_type_defaults)]
#![allow(unused_assignments, static_mut_refs)]

pub mod basic;
pub mod layout;

pub mod softui;
pub use softui::*;

#[cfg(test)]
mod tests;

pub use layout::*;

// use softui::Color;
// pub use softui::{Primative, Rect, Widget};

//An example widget for testing.

pub fn header() -> Header {
    Header {
        title: "title",
        area: Rect::default(),
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub title: &'static str,
    pub area: Rect,
}

impl Widget for Header {
    fn area(&self) -> Rect {
        self.area
    }

    fn primative(&self) -> Primative {
        Primative::Text(String::new(), 0, Color(0))
    }

    // fn on_click<F: FnMut(&mut Self)>(self, button: Button, f: F) -> Click<Self, F> {
    //     Click {
    //         widget: self,
    //         click: (button, f),
    //     }
    // }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

// pub fn widget<T: Widget>(widget: &mut T) -> &mut T {
//     widget
// }

#[macro_export]
macro_rules! count_expr {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        1 + count_expr!($($rest),*)
    };
}
