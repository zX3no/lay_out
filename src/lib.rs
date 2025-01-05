#![feature(associated_type_defaults)]
#![allow(unused_assignments, static_mut_refs)]

pub mod basic;
pub mod layout;

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

#[derive(Copy, Clone, Debug)]
pub enum Button {
    Left,
    Right,
}

pub fn widget<T: Widget>(widget: &mut T) -> &mut T {
    widget
}

#[macro_export]
macro_rules! count_expr {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        1 + count_expr!($($rest),*)
    };
}

//Does not take in references.
#[macro_export]
macro_rules! tlayout {
    (($widget:expr),*) => {};
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug)]
pub enum Primative {
    Rectangle,
    Text(String, usize, Color),
}

#[derive(Copy, Clone, Debug)]
pub struct Color(pub u32);

pub trait Widget
where
    Self: Sized,
{
    type Layout = Self;

    fn area(&self) -> Rect;
    fn area_mut(&mut self) -> Option<&mut Rect>;
    fn primative(&self) -> Primative;
    // fn try_click(&mut self, area: Rect) {}
    fn try_click(&mut self) {}
    fn on_click<F: FnMut(&mut Self)>(self, button: Button, click_fn: F) -> Click<Self, F>
    where
        Self: Sized,
    {
        Click {
            widget: self,
            click: (button, click_fn),
        }
    }
    ///Turns all widget types into a slice so they can be concatenated for layouting.
    #[inline]
    fn as_uniform_layout_type(&mut self) -> &mut [Self::Layout] {
        //Not sure why the type system cannot figure this one out?
        unsafe { core::mem::transmute(core::slice::from_mut(self)) }
    }

    fn x(mut self, x: usize) -> Self {
        let area = self.area_mut().unwrap();
        area.x = x;
        self
    }
    fn y(mut self, y: usize) -> Self {
        let area = self.area_mut().unwrap();
        area.y = y;
        self
    }
}

pub struct Click<T: Widget, F: FnMut(&mut T)> {
    pub widget: T,
    pub click: (Button, F),
}

pub fn run<W: Widget>(w: &mut W) {}

#[macro_export]
macro_rules! tflex {
    ($($widget:expr),* $(,)?) => {{
        $(
            let w = $widget.as_uniform_layout_type();
            // let w = widget(&mut $widget);
            // run(w);
        )*
    }};
}
