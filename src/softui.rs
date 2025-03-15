use crate::Rectangle;

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

pub trait Widget
where
    Self: Sized,
{
    //NOTE: Nightly associated type default.
    type Layout = Self;

    #[must_use]
    fn primative(&self) -> Primative;

    ///Turns all widget types into a slice so they can be concatenated for layouting.
    #[inline]
    fn as_uniform_layout_type(&self) -> &[Self::Layout] {
        //Not sure why the type system cannot figure this one out?
        unsafe { core::mem::transmute(core::slice::from_ref(self)) }
    }

    //TODO: Remove me
    fn as_mut_slice(&mut self) -> &mut [Self]
    where
        Self: Sized,
    {
        core::slice::from_mut(self)
    }

    // fn into_vec(self) -> Vec<Self>
    // where
    //     Self: Sized,
    // {
    //     vec![self]
    // }

    //This one copies
    fn area(&self) -> Rect;
    //This one does not
    fn area_mut(&mut self) -> Option<&mut Rect>;

    // #[inline]
    // fn on_click<F: FnMut(&mut Self)>(self, button: MouseButton, click_fn: F) -> Click0<Self, F>
    // where
    //     Self: Sized,
    // {
    //     Click0 {
    //         widget: self,
    //         //Yes the comma is necassary.
    //         click: ((button, click_fn),),
    //     }
    // }

    #[inline]
    unsafe fn as_mut_ptr(&mut self) -> *mut Self {
        self
    }

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

    //This is used to run the click closure after calling on_click
    //This should be hidden from the user and only implemented on `Click`.
    //https://stackoverflow.com/questions/77562161/is-there-a-way-to-prevent-a-struct-from-implementing-a-trait-method
    #[inline]
    fn try_click(&mut self) {}

    /// The user's cusor has been clicked and released on top of a widget.
    // fn clicked(&mut self, button: MouseButton) -> bool
    // where
    //     Self: Sized,
    // {
    //     let ctx = ctx();
    //     let area = self.area();

    //     if !ctx.mouse_pos.intersects(area) {
    //         return false;
    //     }

    //     match button {
    //         MouseButton::Left => {
    //             ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Right => {
    //             ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Middle => {
    //             ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Back => {
    //             ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
    //         }
    //         MouseButton::Forward => {
    //             ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
    //         }
    //     }
    // }
    // fn up(&mut self, button: MouseButton) -> bool
    // where
    //     Self: Sized,
    // {
    //     let ctx = ctx();
    //     let area = self.area_mut().unwrap().clone();
    //     if !ctx.mouse_pos.intersects(area) {
    //         return false;
    //     }

    //     match button {
    //         MouseButton::Left => ctx.left_mouse.released,
    //         MouseButton::Right => ctx.right_mouse.released,
    //         MouseButton::Middle => ctx.middle_mouse.released,
    //         MouseButton::Back => ctx.mouse_4.released,
    //         MouseButton::Forward => ctx.mouse_5.released,
    //     }
    // }
    // fn down(&mut self, button: MouseButton) -> bool
    // where
    //     Self: Sized,
    // {
    //     let ctx = ctx();
    //     let area = self.area_mut().unwrap().clone();
    //     if !ctx.mouse_pos.intersects(area) {
    //         return false;
    //     }

    //     match button {
    //         MouseButton::Left => ctx.left_mouse.pressed,
    //         MouseButton::Right => ctx.right_mouse.pressed,
    //         MouseButton::Middle => ctx.middle_mouse.pressed,
    //         MouseButton::Back => ctx.mouse_4.pressed,
    //         MouseButton::Forward => ctx.mouse_5.pressed,
    //     }
    // }

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
