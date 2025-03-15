#![allow(unused)]
#![feature(box_as_ptr)]
// use lay_out::*;

use std::{
    cell::UnsafeCell,
    mem::transmute,
    ops::{Add, DerefMut},
    pin::{pin, Pin},
    ptr::addr_of_mut,
};

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
    fn into_ui<'a, const N: usize>(self, parent: *mut UIElement<'a, N>) -> Box<UIElement<'a, N>> {
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
struct UIElement<'a, const N: usize> {
    //Width and height of 0 will use fit sizing.
    area: Rect,
    direction: Direction,
    padding: Padding,
    //Temp bypass
    parent: Option<*mut UIElement<'a, N>>,
    children: Option<[&'a UIElement<'a, N>; N]>,
    gap: usize,
}

fn main() {
    //32, 32
    //root is box for static
    let mut r = UIElement {
        // area: Rect::new(0, 0, 960, 540),
        area: Rect::new(0, 0, 0, 0),
        parent: None,
        direction: Direction::LeftRight,
        padding: Padding::new(32, 32, 32, 32),
        // children: Some([&red, &yellow]),
        children: None,
        gap: 32,
    };

    let mut root = Pin::new(&mut r);

    //A Pin<Ptr> does not pin the Ptr but rather the pointer’s pointee value.
    //In this case (&mut UIElement) is not pinned but rather the UIElement.
    //It's unclear how we can bypass the aliasing rules with this.
    //I have a feeling that Pin does not support this and I'll need to think of
    //a safer way of doing this.
    //TODO: Read through https://doc.rust-lang.org/std/pin/index.html
    let root_ptr = root.deref_mut() as *mut _;

    let mut red = unsafe { Rect::new(0, 0, 300, 300).into_ui(root_ptr) };
    let mut yellow = unsafe { Rect::new(0, 0, 350, 200).into_ui(root_ptr) };
    root.children = Some([&red, &yellow]);

    for child in root.children.unwrap() {
        root.area.width += root.padding.left + root.padding.right;
        root.area.height += root.padding.top + root.padding.bottom;

        let gap = (root.children.unwrap().len() - 1) * root.gap;

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
            for child in root.children.unwrap() {
                let x = root.area.x + child.area.x + left_offset;
                let y = root.area.y + child.area.y + root.padding.top;
                println!("x:{x} y:{y}");
                left_offset += child.area.width + root.gap;
            }
        }
        Direction::TopBottom => {
            let mut top_offset = root.padding.top;
            for child in root.children.unwrap() {
                let x = root.area.x + child.area.x + root.padding.left;
                let y = root.area.y + child.area.y + top_offset;
                println!("x:{x} y:{y}");
                top_offset += child.area.height + root.gap;
            }
        }
        _ => todo!(),
    }
}
