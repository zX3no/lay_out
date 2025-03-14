#![allow(unused)]

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

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    value: Option<Primative>,
}

fn rect() -> Node {
    Node {
        children: Vec::new(),
        value: Some(Primative::StaticText("rect")),
    }
}

fn text() -> Node {
    Node {
        children: Vec::new(),
        value: Some(Primative::StaticText("text")),
    }
}

fn image() -> Node {
    Node {
        children: Vec::new(),
        value: Some(Primative::StaticText("image")),
    }
}

fn svg() -> Node {
    Node {
        children: Vec::new(),
        value: Some(Primative::StaticText("svg")),
    }
}

fn main() {
    let tree = v!(rect(), text(), v!(image(), svg()));
    let tree2 = v!(rect());
    dbg!(&tree, tree2);
}
