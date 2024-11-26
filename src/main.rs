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
        Click { widget: self, click: (button, f) }
    }

    // type Layout = Self;

    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
}

fn main() {
    let mut header = Header {
        title: "hi",
        area: Rect { x: 0, y: 0, width: 20, height: 20 },
    };

    let mut header2 = Header {
        title: "hi",
        area: Rect { x: 0, y: 0, width: 20, height: 20 },
    };

    unsafe {
        VIEWPORT_HEIGHT = 20;
        VIEWPORT_WIDTH = 40;
    }

    let test = layout!(header, header2);
    assert!(test.len() == 2);
    assert!(test[0].0.x == 0);
    assert!(test[1].0.x == 20);
}
