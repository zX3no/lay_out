#![allow(unused)]
use crate::{Color, Primative, Rect, Widget};

pub fn rect() -> Rectangle {
    Rectangle::default()
}

#[derive(Debug, Default, Clone)]
pub struct Rectangle {
    area: Rect,
    padding: Padding,
    color: Color,
}

impl Rectangle {
    pub fn padding(mut self, padding: (usize, usize, usize, usize)) -> Self {
        self.padding = padding.into();
        self
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Rectangle {
    type Layout = Self;

    fn primative(&self) -> Primative {
        Primative::Ellipse(0, self.color)
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

pub fn draw_call(primative: Primative) {}

#[derive(Default, Debug, Clone, Copy)]
pub enum FlexDirection {
    #[default]
    LeftRight,
    RightLeft,
    TopBottom,
    BottomTop,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Padding {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl From<(usize, usize, usize, usize)> for Padding {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl Padding {
    pub const fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

struct Draw {}

pub fn grow<W: Widget>(
    widget: &mut W,
    x: &mut usize,
    y: &mut usize,
    max_width: &mut usize,
    max_height: &mut usize,
    direction: FlexDirection,
) {
}

pub fn grow2<W: Widget>(widget: &mut W, offset: &mut usize, gap: usize, direction: FlexDirection) {
    let area = widget.area_mut().unwrap();
    match direction {
        FlexDirection::LeftRight => {
            area.x += *offset;
            *offset += area.width + gap;
        }
        _ => todo!(),
        // FlexDirection::RightLeft => {
        //     area.x -= *offset;
        //     *offset += area.width + gap;
        // }
        // FlexDirection::TopBottom => {
        //     area.y += *offset;
        //     *offset += area.height + gap;
        // }
        // FlexDirection::BottomTop => {
        //     area.y -= *offset;
        //     *offset += area.height + gap;
        // }
    }
}

pub fn calculate_offset(direction: FlexDirection, padding: Padding) -> usize {
    match direction {
        FlexDirection::LeftRight => padding.left,
        FlexDirection::RightLeft => padding.right,
        FlexDirection::TopBottom => padding.top,
        FlexDirection::BottomTop => padding.bottom,
    }
}
#[macro_export]
macro_rules! grow {
    ($widget:expr) => {{
        let w = &mut $widget;
        $crate::draw_call(w.primative());
    }};
    ($($widget:expr),* $(,)?) => {{
        let mut x = 0;
        let mut y = 0;
        let mut max_height = 0;
        let mut max_width = 0;

        let direction = $crate::FlexDirection::LeftRight;

        let padding = $crate::Padding::new(32, 32, 32, 32);
        let mut offset = $crate::calculate_offset(direction, padding);

        let gap = 32;

        $(
            let w = &mut $widget;
            // $crate::grow(w, &mut x, &mut y, &mut max_width, &mut max_height, direction);
            $crate::grow2(w, &mut offset, gap, direction);
        )*
    }};
}

#[derive(Debug)]
pub struct Container {
    pub widgets: Vec<(Rect, Primative)>,
    pub area: Rect,
}

impl Widget for Container {
    type Layout = Self;

    fn primative(&self) -> crate::Primative {
        todo!()
    }

    fn area(&self) -> crate::Rect {
        todo!()
    }

    fn area_mut(&mut self) -> Option<&mut crate::Rect> {
        todo!()
    }
}

// +---------------------------------+
// | +-------------+ +-------------+ |
// | |             | |             | |
// | |             | |             | |
// | |             | +-------------+ |
// | |             |                 |
// | +-------------+                 |
// +------------------------+--------+

//flex!(
//    h!(rect().wh(300), rect().w(300).h(200)).gap(32)
//).padding(32)

//First let's start with the child container.
//h!(rect().wh(300), rect().w(300).h(200)).gap(32)
//We are not calculating position with this.
//Instead we want to know the size of the container
//We already know the size of each widget.

//For this example Width = 300 + 32 + 300 = 632
//                 Height = 300

//Now this should be passed as a widget onto the next macro.
//I think that the widget trait will need to be reworked.
//Widget.primative() should return &[Primative] not a single Primative.
//This is because widgets need to be able to hold multiple other widgets.
//We also have the Widget.is_container() function.

//Size the elements first and then set the positions later.
#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        //TODO: Builder for this.
        let gap = 32;
        let count = $crate::count_expr!($($widget),*);
        let gap = (count - 1) * gap;

        let mut width = gap;
        let mut height = 0;

        let mut widgets = Vec::new();

        $(
            let w = &mut $widget;
            let area = w.area();
            height = area.height.max(height);
            width += area.width;
            widgets.push((area, w.primative()));
        )*

        Container { widgets, area: Rect::new(0, 0, width, height) }
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_two_rect() {
        let container = h!(rect().wh(300), rect().w(300).h(200));
        assert!(container.area.width == 632);
        assert!(container.area.height == 300);
        assert!(container.widgets.len() == 2)
    }
}
