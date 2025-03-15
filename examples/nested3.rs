#![allow(unused, internal_features)]
#![feature(const_type_id, core_intrinsics)]

macro_rules! v {
    ($elem:expr) => {
        $elem
    };

    //At least one element with optional trailing comma.
    ($($elem:expr),+ $(,)?) => {
        {
            let mut parent = Node { children: Vec::new(), value: None };

            $(
                parent.children.push($elem);
            )+

            parent
        }
    };
}

#[derive(Debug)]
enum Primative {
    StaticText(&'static str),
}

struct Tree(Node);

#[derive(Default, Debug)]
struct Node {
    children: Vec<Node>,
    value: Option<Primative>,
}

// impl Drop for Node {
//     fn drop(&mut self) {
//         dbg!(self);
//     }
// }

fn rect() -> Node {
    Node::default()
}

fn text() -> Node {
    Node::default()
}

fn image() -> Node {
    Node::default()
}

fn svg() -> Node {
    Node::default()
}

fn extend_lifetime<'a, T>(t: &'a T) -> &'static T {
    unsafe { std::mem::transmute::<&'a T, &'static T>(t) }
}

const fn is_tree_container<T: 'static>(value: &T) -> bool {
    std::intrinsics::type_id::<T>() == std::intrinsics::type_id::<Node>()
}

macro_rules! parse {
    ($e:expr) => {{
        dbg!($e);
    }};
    ($e:expr, $($tail:tt)*) => {{
        let mut parent = Node {
            children: Vec::new(),
            value: None,
        };
    }};
}

fn main() {
    // let tree = v!(rect(), text(), v!(image(), svg()));
    // let tree2 = v!(rect());

    parse!(rect(), text(), parse!(image(), svg()))
}
