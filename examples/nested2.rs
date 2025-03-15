#![allow(unused)]
use lay_out::*;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default, Debug, Clone)]
struct Node {
    area: Rect,
    direction: Direction,
    // padding: Padding,
    gap: usize,
    index: usize,
    parent: Option<usize>,
    //TODO: This should be a small vec.
    children: Vec<usize>,
}

fn add_node(arena: &mut Vec<Node>, mut node: Node, parent: Option<usize>) -> usize {
    let idx = arena.len();
    node.index = idx;
    node.parent = parent;

    //Add the child node to the parent
    if let Some(parent) = node.parent {
        if let Some(parent) = arena.get_mut(parent) {
            parent.children.push(idx);
        }
    }

    arena.push(node);
    idx
}

struct DepthFirstSearchIter<'a> {
    arena: &'a [Node],
    stack: Vec<usize>,
}

impl<'a> DepthFirstSearchIter<'a> {
    fn new(arena: &'a [Node], root: usize) -> Self {
        Self {
            arena,
            stack: vec![root],
        }
    }
}

impl<'a> Iterator for DepthFirstSearchIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node_idx = self.stack.pop()?;
        let node = self.arena.get(node_idx)?;
        self.stack.extend(node.children.iter().rev());
        Some(node)
    }
}

static mut ARENA: Vec<Node> = Vec::new();

macro_rules! root {
    ($($widget:expr),* $(,)?) => {{
        let widgets: Vec<Node> = Vec::new();
    }};
}

fn rect() -> Node {
    Node::default()
}

fn main() {
    root!(rect(), rect());
}
