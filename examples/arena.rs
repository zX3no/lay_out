#![allow(unused)]
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Add, Deref, DerefMut},
    ptr::addr_of_mut,
};

use mini::{defer_results, profile};

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub enum Direction {
    #[default]
    LeftRight,
    TopBottom,
    RightLeft,
    BottomTop,
}

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
    // fn into_ui<'a, const N: usize>(self, parent: *mut UIElement<'a, N>) -> Box<UIElement<'a, N>> {
    //     Box::new(UIElement {
    //         area: self,
    //         parent: Some(parent),
    //         ..Default::default()
    //     })
    // }
}

#[derive(Default)]
struct Arena(Vec<Node>);

impl Deref for Arena {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Arena {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Arena {
    fn get_node(&self, node: usize) -> Option<&Node> {
        self.get(node)
    }
    fn add_node(&mut self, parent: Option<usize>) -> usize {
        let mut node = Node::default();
        let idx = self.len();
        node.index = idx;
        node.parent = parent;

        if let Some(parent) = parent {
            if let Some(parent) = self.get_mut(parent) {
                parent.children.push(idx);
            }
        }

        self.push(node);
        idx
    }
}

// struct ParentNode(usize);

#[derive(Default, Debug)]
struct Node {
    area: Rect,
    direction: Direction,
    padding: Padding,
    gap: usize,
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

fn main() {
    defer_results!();

    let len = 10;
    assert_eq!(len << 1, (len << 1) | false as usize);

    let mut ar = Arena::default();

    let parent = ar.add_node(None);

    for _ in 0..1_000_000 {
        ar.add_node(Some(parent));
    }

    profile!();

    let p = ar.get_node(parent).unwrap();
    for child in &p.children {
        let node = ar.get_node(*child).unwrap();
        assert!(node.area.x == 0);
    }

    // let child1 = arena.add_node(Some(parent));
    // let child2 = arena.add_node(Some(parent));

    // dbg!(arena.get_node(parent).unwrap().area);
    // dbg!(arena.get_node(child1).unwrap().area);
    // dbg!(arena.get_node(child2).unwrap().area);
}
