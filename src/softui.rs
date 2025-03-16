use std::fmt::Display;

pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    ///https://www.w3schools.com/cssref/css_units.php
    Em(usize),
    //Percentage relative to what?
    Percentage(usize),
}

impl From<usize> for Unit {
    fn from(val: usize) -> Self {
        Unit::Px(val)
    }
}

impl From<i32> for Unit {
    fn from(value: i32) -> Self {
        Unit::Px(value.try_into().unwrap())
    }
}

impl From<f32> for Unit {
    fn from(val: f32) -> Self {
        Unit::Percentage((val * 100.0) as usize)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    // pub const fn default() -> Self {
    //     Self {
    //         x: 0,
    //         y: 0,
    //         width: 0,
    //         height: 0,
    //     }
    // }
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub const fn right(&self) -> usize {
        self.x + self.width
    }
    pub const fn bottom(&self) -> usize {
        self.y + self.height
    }
    // pub const fn centered(&self, width: u16, height: u16) -> Rect {
    //     let v = self.width() / 2;
    //     let h = self.height() / 2;

    //     todo!();
    // }
    // pub const fn area(&self) -> usize {
    //     self.width * self.height
    // }

    //TODO: Write some tests.
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    //TODO: Bounds checking
    pub const fn inner(&self, w: usize, h: usize) -> Rect {
        Rect {
            x: self.x + w,
            y: self.y + h,
            width: self.width - 2 * w,
            height: self.height - 2 * h,
        }
    }

    #[inline]
    pub const fn closure(&self) -> impl Fn(Rect) -> Rect {
        |rect: Rect| rect
    }

    // pub const fn inner(self, w: u16, h: u16) -> Result<Rect, &'static str> {
    //     if self.width < 2 * w {
    //         Err("Inner area exceeded outside area. Reduce margin width.")
    //     } else if self.height < 2 * h {
    //         Err("Inner area exceeded outside area. Reduce margin height.")
    //     } else {
    //         Ok(Rect {
    //             x: self.x + w,
    //             y: self.y + h,
    //             width: self.width - 2 * w,
    //             height: self.height - 2 * h,
    //         })
    //     }
    // }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, width: {}, height: {}",
            self.x, self.y, self.width, self.height
        )
    }
}

impl Widget for Rect {
    type Layout = Self;

    fn primative(&self) -> Primative {
        todo!()
    }

    fn area(&self) -> Rect {
        *self
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(self)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    ///Mouse4
    Back,
    ///Mouse5
    Forward,
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(pub u32);

#[derive(Debug, Clone)]
pub enum Primative {
    ///Radius, Color
    Ellipse(usize, Color),
    RectangleOutline(Color),
    Text(String, usize, Color),
}

impl Display for Primative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primative::Ellipse(_, _) => write!(f, "Ellipse"),
            Primative::RectangleOutline(_) => write!(f, "RectangleOutline"),
            Primative::Text(_, _, _) => write!(f, "Text"),
        }
    }
}

pub trait Widget
where
    Self: Sized,
{
    //NOTE: Nightly associated type default.
    type Layout = Self;

    #[must_use]
    fn primative(&self) -> Primative;

    #[inline]
    fn as_container_slice_mut(&mut self) -> &mut [(Rect, Primative)] {
        unreachable!("This function should never be called on non-container types.")
    }

    //This one copies
    fn area(&self) -> Rect;
    //This one does not
    fn area_mut(&mut self) -> Option<&mut Rect>;

    //This should be called need_draw, need_compute_area, idk...
    //If we used Any we could just call self.type_id() == Container.
    //Easy as that.
    #[inline]
    fn is_container() -> bool
    where
        Self: Sized,
    {
        false
    }

    fn centered(mut self, parent: Rect) -> Self
    where
        Self: Sized,
    {
        let parent_area = parent.clone();
        let area = self.area_mut().unwrap();
        let x = (parent_area.width as f32 / 2.0) - (area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (area.height as f32 / 2.0);

        *area = Rect::new(
            x.round() as usize,
            y.round() as usize,
            area.width,
            area.height,
        );

        self
    }
    fn x<U: Into<Unit>>(mut self, x: U) -> Self
    where
        Self: Sized,
    {
        let area = self.area_mut().unwrap();
        match x.into() {
            Unit::Px(px) => {
                area.x = px;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => {
                todo!();
                // let percentage = p as f32 / 100.0;
                // area.x = ((self.parent_area.width as f32 * percentage)
                //     - (self.area.width as f32 / 2.0))
                //     .round() as i32;
            }
        }
        self
    }
    fn y<U: Into<Unit>>(mut self, y: U) -> Self
    where
        Self: Sized,
    {
        match y.into() {
            Unit::Px(px) => {
                self.area_mut().unwrap().y = px;
                // self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn width<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.area_mut().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.width = px;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn height<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.area_mut().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.height = px;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn w<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.width(width)
    }
    fn h<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.height(width)
    }
    //Swizzle 😏
    fn wh<U: Into<Unit> + Copy>(self, value: U) -> Self
    where
        Self: Sized,
    {
        self.width(value).height(value)
    }
    fn top<U: Into<Unit>>(self, top: U) -> Self
    where
        Self: Sized,
    {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self
    where
        Self: Sized,
    {
        self.x(left)
    }
    // fn right<U: Into<Unit>>(mut self, length: U) -> Self
    // where
    //     Self: Sized,
    // {
    //     match length.into() {
    //         Unit::Px(px) => todo!(),
    //         Unit::Em(_) => todo!(),
    //         Unit::Percentage(_) => todo!(),
    //     }
    //     self
    // }
    // fn bottom<U: Into<Unit>>(mut self, length: U) -> Self
    // where
    //     Self: Sized,
    // {
    //     match length.into() {
    //         Unit::Px(px) => todo!(),
    //         Unit::Em(_) => todo!(),
    //         Unit::Percentage(_) => todo!(),
    //     }
    //     self
    // }
    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self
    where
        Self: Sized,
    {
        self.x(x).y(y).width(width).height(height)
    }
}
