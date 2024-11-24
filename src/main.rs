use lay_out::*;

#[derive(Clone, Debug)]
pub struct Header {
    pub title: &'static str,
    pub area: Rect,
}

impl Widget for Header {
    fn area(&self) -> lay_out::Rect {
        self.area
    }

    fn primative(&self) -> Primative {
        Primative::Text
    }

    fn on_click<F: FnMut(&mut Self)>(self, button: Button, f: F) -> Click<Self, F> {
        Click {
            widget: self,
            click: (button, f),
        }
    }
}

fn main() {
    let mut header = Header {
        title: "hi",
        area: Rect::default(),
    };

    let mut header2 = Header {
        title: "hi",
        area: Rect::default(),
    };

    layout!(header, header2);
}
