#![allow(unused)]
use crate::{Color, Primative, Rect, Widget};

#[macro_export]
macro_rules! count_expr {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        1 + $crate::count_expr!($($rest),*)
    };
}

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

pub fn calculate_offset(direction: FlexDirection, padding: Padding) -> usize {
    match direction {
        FlexDirection::LeftRight => padding.left,
        FlexDirection::RightLeft => padding.right,
        FlexDirection::TopBottom => padding.top,
        FlexDirection::BottomTop => padding.bottom,
    }
}

// Positioning Step
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

//If no x and y values are specified start at the top left (0, 0)
//Assuming left to right layout.
//Start the x value at the left padding value.
//In this case our x value is `offset = 32`
//Our first widget is a container.
//We need to calculate the positioning of each of the widgets.
//The first will be at x = 32,
//the second will be at x = 32 + widget.width + gap
//The parent size will be padding.left + container.width + padding.right

//Because of the way macros work, the flex macro will need to also do sizing
//I can't chain together v!() and h!() without something delimiting the root node.
//There might be a way but I've been working on this for too long it's time to LOCK IN.

pub fn calculate_sizing(container: &mut Container, area: &mut Rect, direction: FlexDirection) {
    match direction {
        FlexDirection::LeftRight => {
            area.width += container.area.width;
            area.height = area.height.max(container.area.height)
        }
        FlexDirection::RightLeft => todo!(),
        FlexDirection::TopBottom => todo!(),
        FlexDirection::BottomTop => todo!(),
    }
}

pub fn draw_call(area: Rect, primative: Primative) {
    println!("Drawing widget({}) at {}", primative, area);
}

pub fn draw_widgets(
    container: &mut Container,
    direction: FlexDirection,
    offset: &mut usize,
    gap: usize,
) {
    for (area, primative) in &container.widgets {
        let mut area = area.clone().x(*offset);
        draw_call(area, primative.clone());
        *offset += area.width + gap;
    }
}

pub fn debug_draw_widgets(
    debug: &mut Vec<(Rect, Primative)>,
    container: &mut Container,
    direction: FlexDirection,
    offset: &mut usize,
    gap: usize,
) {
    for (area, primative) in &container.widgets {
        let mut area = area.x(*offset);
        debug.push((area, primative.clone()));
        draw_call(area, primative.clone());
        *offset += area.width + gap;
    }
}



#[rustfmt::skip] 
#[macro_export]
macro_rules! flex {
    //Assume eveything being pass in is a container.
    ($($container:expr),* $(,)?) => {{ 
        let f = |direction: $crate::FlexDirection, padding: $crate::Padding, gap: usize| {
            let mut offset = $crate::calculate_offset(direction, padding);
            let mut area = Rect::default();

            $(
                let mut container = $container.call();
                $crate::calculate_sizing(&mut container, &mut area, direction);
                $crate::draw_widgets(&mut container, direction, &mut offset, gap);
            )*

            area
        };
        $crate::DeferFlex {
            f,
            direction: $crate::FlexDirection::LeftRight,
            padding: $crate::Padding::default(),
            gap: 0,
        }
    }}

    // ($($widget:expr),* $(,)?) => {{
    //     let mut x = 0;
    //     let mut y = 0;
    //     let mut width = 0;
    //     let mut height = 0;

    //     let direction = $crate::FlexDirection::LeftRight;
    //     let padding = $crate::Padding::new(32, 32, 32, 32);
    //     //Padding is just the directional padding.
    //     let mut offset = $crate::calculate_offset(direction, padding);

    //     let gap = 32;

    //     $(
    //         let w = &mut $widget;
    //         $crate::grow(w, &mut offset, gap, direction);
    //     )*
    // }};
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
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut crate::Rect> {
        Some(&mut self.area)
    }

    fn is_container() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn as_container_slice_mut(&mut self) -> &mut [(Rect, Primative)] {
        &mut self.widgets
    }
}

//Sizing -> Positioning -> Rendering

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

//For this example width = 300 + 32 + 300 = 632
//                 height = 300

//Now this should be passed as a widget onto the next macro.
//I think that the widget trait will need to be reworked.
//Widget.primative() should return &[Primative] not a single Primative.
//This is because widgets need to be able to hold multiple other widgets.
//We also have the Widget.is_container() function.

//Size the elements first and then set the positions later.
#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        //TODO: Padding is unused here.
        let f = |padding: Padding, gap: usize| {
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
        };

        //Defer the creation of the container so that the builder pattern
        //can be used to modifiy aspects of the container such as gap and padding.
        $crate::DeferContainer {
            f,
            padding: Padding::default(),
            gap: 0,
        }
    }};
}

//Maybe group into one struct????
//Could also convert into widget to simplify calling code.
pub struct DeferFlex<F> {
    pub f: F,
    pub direction: FlexDirection,
    pub padding: Padding,
    pub gap: usize,
}

impl<F> DeferFlex<F> {
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }
    //TODO: Padding left, right, etc.
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = Padding::new(padding, padding, padding, padding);
        self
    }
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<F> Defer for DeferFlex<F>
where
    F: Fn(FlexDirection, Padding, usize) -> Rect,
{
    type T = Rect;
    fn call(&self) -> Self::T {
        (self.f)(self.direction, self.padding, self.gap)
    }
}

pub struct DeferContainer<F> {
    pub f: F,
    pub padding: Padding,
    pub gap: usize,
}

impl<F> DeferContainer<F> {
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = Padding::new(padding, padding, padding, padding);
        self
    }
}
impl<F> Defer for DeferContainer<F>
where
    F: Fn(Padding, usize) -> Container,
{
    type T = Container;
    fn call(&self) -> Self::T {
        (self.f)(self.padding, self.gap)
    }
}

pub trait Defer {
    type T;
    fn call(&self) -> Self::T;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_two_rect() {
        let mut container = h!(rect().wh(300), rect().w(300).h(200)).gap(32).call();
        assert!(container.area.width == 632);
        assert!(container.area.height == 300);
        assert!(container.widgets.len() == 2);

        let gap = 32;
        let mut offset = gap;
        let mut debug = Vec::new();
        debug_draw_widgets(&mut debug, &mut container, FlexDirection::LeftRight, &mut offset, gap);

        assert!(debug[0].0.x == 32);
        assert!(debug[1].0.x == 32 + 300 + 32);

        let flex = flex!(h!(rect().wh(300), rect().w(300).h(200)).gap(32))
            .padding(32)
            .gap(32)
            .call();

        assert!(flex.width == 632);
        assert!(flex.height == 300);
    }
}
