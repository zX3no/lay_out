#![allow(unused)]
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Add, Deref, DerefMut},
    ptr::addr_of_mut,
};

use mini::{defer_results, profile};

#[derive(Default, Debug, Clone, Copy)]
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

#[derive(Default, Debug, Clone, Copy)]
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
    fn into_node(self) -> Node {
        Node {
            area: self,
            ..Default::default()
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

//I'll keep it simple for now,
//there are likely ways to improve this.
//TODO: This should be a small vec
//1000 items would be 16KB (128 * 1000 / 8 / 1000 = 16)
#[derive(Default, Debug)]
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
    fn get_node_mut(&mut self, node: usize) -> Option<&mut Node> {
        self.get_mut(node)
    }
    fn create_node(&mut self, parent: Option<usize>) -> usize {
        let mut node = Node::default();
        self.add_node(node, parent)
    }
    fn add_node(&mut self, mut node: Node, parent: Option<usize>) -> usize {
        let idx = self.len();
        node.index = idx;
        node.parent = parent;

        //Add the child node to the parent
        if let Some(parent) = node.parent {
            if let Some(parent) = self.get_mut(parent) {
                parent.children.push(idx);
            }
        }

        self.push(node);
        idx
    }
}

//It is important that nodes are as small as possible
//Currently they are 128bits
#[derive(Default, Debug, Clone)]
struct Node {
    area: Rect,
    direction: Direction,
    padding: Padding,
    gap: usize,
    index: usize,
    parent: Option<usize>,
    //TODO: This should be a small vec.
    children: Vec<usize>,
}

//This is probably less optimal than using reccursion.
//I'm guessing because of the extra allocations.
struct DfsIter<'a> {
    arena: &'a Arena,
    stack: Vec<usize>,
}

impl<'a> DfsIter<'a> {
    fn new(arena: &'a Arena, root: usize) -> Self {
        Self {
            arena,
            stack: vec![root],
        }
    }
}

impl<'a> Iterator for DfsIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node_idx = self.stack.pop()?;
        let node = self.arena.get_node(node_idx)?;
        self.stack.extend(node.children.iter().rev());
        Some(node)
    }
}

//https://stackoverflow.com/questions/65948553/why-is-recursion-not-suggested-in-rust
//Tail call elimination is missed when calling this, I have checked the assembly it uses `call example::traverse::h58697ea0ab44a8ac`
fn traverse(arena: &Arena, node_idx: usize) {
    if let Some(node) = arena.get_node(node_idx) {
        println!("Visiting node: {:?}", node);

        for &child_idx in &node.children {
            traverse(arena, child_idx);
        }
    }
}

fn node_test() {
    let mut ar = Arena::default();

    let parent = ar.create_node(None);

    let mut red = Rect::new(0, 0, 300, 300).into_node();
    let mut yellow = Rect::new(0, 0, 350, 200).into_node();

    let red = ar.add_node(red, Some(parent));
    let yellow = ar.add_node(yellow, Some(parent));
    ar.add_node(Rect::new(0, 0, 20, 20).into_node(), Some(red));

    // for node in DfsIter::new(&ar, parent) {
    //     println!("{:?}", node);
    // }

    traverse(&ar, parent);
}

fn main() {
    defer_results!();
    return node_test();

    let mut ar = Arena::default();

    let parent = ar.create_node(None);

    let mut red = Rect::new(0, 0, 300, 300).into_node();
    let mut yellow = Rect::new(0, 0, 350, 200).into_node();

    ar.add_node(red, Some(parent));
    ar.add_node(yellow, Some(parent));

    //If we want to avoid the clone here,
    //we need to guarentee that `ar[parent_idx].children` is not changed while iterating.
    for child_idx in ar[parent].children.clone() {
        let [root, child] = unsafe { ar.get_disjoint_unchecked_mut([parent, child_idx]) };

        root.area.width += root.padding.left + root.padding.right;
        root.area.height += root.padding.top + root.padding.bottom;

        let gap = (root.children.len() - 1) * root.gap;

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

    //Not liking the need to index every time here to avoid aliasing issues.
    //It's a shame the complier is unable to figure this out.
    match ar[parent].direction {
        Direction::LeftRight => {
            let mut left_offset = ar[parent].padding.left;
            for child in ar[parent].children.clone() {
                let [root, child] = ar.get_disjoint_mut([parent, child]).unwrap();
                let x = root.area.x + child.area.x + left_offset;
                let y = root.area.y + child.area.y + root.padding.top;
                println!("x:{x} y:{y}");
                left_offset += child.area.width + root.gap;
            }
        }
        Direction::TopBottom => {
            let mut top_offset = ar[parent].padding.top;
            for child in ar[parent].children.clone() {
                let [root, child] = ar.get_disjoint_mut([parent, child]).unwrap();
                let x = root.area.x + child.area.x + root.padding.left;
                let y = root.area.y + child.area.y + top_offset;
                println!("x:{x} y:{y}");
                top_offset += child.area.height + root.gap;
            }
        }
        _ => todo!(),
    }
}
