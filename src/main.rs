#![allow(unused)]
#![feature(box_as_ptr)]
// use lay_out::*;

use std::{cell::UnsafeCell, ops::Add};

fn rect() -> Rect {
    Rect::default()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
impl Add for Rect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Rect {
    const fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
    const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    fn into_ui<'a, const N: usize>(self, parent: *mut UIElement<N>) -> Box<UIElement<N>> {
        Box::new(UIElement {
            area: self,
            parent: Some(parent),
            ..Default::default()
        })
    }
}

#[derive(Default)]
struct Padding {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl Padding {
    const fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

#[derive(Default)]
pub enum Direction {
    #[default]
    LeftRight,
    TopBottom,
    RightLeft,
    BottomTop,
}

#[derive(Default)]
struct UIElement<const N: usize> {
    //Width and height of 0 will use fit sizing.
    area: Rect,
    direction: Direction,
    padding: Padding,
    //Temp bypass
    parent: Option<*mut UIElement<N>>,
    children: Option<[Box<UIElement<N>>; N]>,
    gap: usize,
}
fn main() {
    //32, 32

    let mut root = Box::new(UIElement {
        // area: Rect::new(0, 0, 960, 540),
        area: Rect::new(0, 0, 0, 0),
        parent: None,
        direction: Direction::LeftRight,
        padding: Padding::new(32, 32, 32, 32),
        // children: Some([&red, &yellow]),
        children: None,
        gap: 32,
    });

    unsafe {
        let mut red = Rect::new(0, 0, 300, 300).into_ui(Box::as_mut_ptr(&mut root));
        let mut yellow = Rect::new(0, 0, 350, 200).into_ui(Box::as_mut_ptr(&mut root));
        root.children = Some([red, yellow]);
    }

    for child in root.children.as_ref().unwrap() {
        root.area.width += root.padding.left + root.padding.right;
        root.area.height += root.padding.top + root.padding.bottom;

        let gap = (root.children.as_ref().unwrap().len() - 1) * root.gap;

        match root.direction {
            Direction::LeftRight => {
                root.area.width += child.area.width + gap;
                root.area.height += child.area.height.max(root.area.height);
            }
            Direction::TopBottom => {
                root.area.width += child.area.width.max(root.area.width);
                root.area.height += child.area.height + gap;
            }
            _ => todo!(),
        }
    }

    dbg!(root.area);

    match root.direction {
        Direction::LeftRight => {
            let mut left_offset = root.padding.left;
            for child in root.children.as_ref().unwrap() {
                let x = root.area.x + child.area.x + left_offset;
                let y = root.area.y + child.area.y + root.padding.top;
                println!("x:{x} y:{y}");
                left_offset += child.area.width + root.gap;
            }
        }
        Direction::TopBottom => {
            let mut top_offset = root.padding.top;
            for child in root.children.as_ref().unwrap() {
                let x = root.area.x + child.area.x + root.padding.left;
                let y = root.area.y + child.area.y + top_offset;
                println!("x:{x} y:{y}");
                top_offset += child.area.height + root.gap;
            }
        }
        _ => todo!(),
    }
}
