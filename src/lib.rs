#![feature(associated_type_defaults)]
#![allow(unused_assignments, static_mut_refs)]

//An example widget for testing.
#[derive(Clone, Debug)]
pub struct Header {
    pub title: &'static str,
    pub area: Rect,
}

impl Widget for Header {
    fn area(&self) -> Rect {
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

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum Primative {
    Rectangle,
    Text,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug)]
pub enum Button {
    Left,
    Right,
}

pub trait Widget
where
    Self: Sized,
{
    type Layout = Self;

    fn area(&self) -> Rect;
    fn area_mut(&mut self) -> Option<&mut Rect>;
    fn primative(&self) -> Primative;
    // fn try_click(&mut self, area: Rect) {}
    fn try_click(&mut self) {}
    fn on_click<F: FnMut(&mut Self)>(self, button: Button, click_fn: F) -> Click<Self, F>
    where
        Self: Sized,
    {
        Click {
            widget: self,
            click: (button, click_fn),
        }
    }
    ///Turns all widget types into a slice so they can be concatenated for layouting.
    #[inline]
    fn as_uniform_layout_type(&mut self) -> &mut [Self::Layout] {
        //Not sure why the type system cannot figure this one out?
        unsafe { core::mem::transmute(core::slice::from_mut(self)) }
    }
}

pub struct Click<T: Widget, F: FnMut(&mut T)> {
    pub widget: T,
    pub click: (Button, F),
}

// pub fn clicked<T: Widget>(widget: &mut T, button: Button) {}

pub fn widget<T: Widget>(widget: &mut T) -> &mut T {
    widget
}

//I think I'll need two different macros for `v!(&mut rect)` and `v!(rect())`
#[macro_export]
macro_rules! count_expr {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        1 + count_expr!($($rest),*)
    };
}

//Does not take in references.
#[macro_export]
macro_rules! tlayout {
    (($widget:expr),*) => {};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Center {
    Horizontal,
    Vertical,
    Both,
}

//This does centers all widgets horizontally.
#[macro_export]
macro_rules! flex_center {
    ($center:expr, $vw:expr, $vh:expr, $($widget:expr),*) => {{
        // let mut widgets = Vec::new();

        // $(
        //     let w = widget(&mut $widget);
        //     widgets.push((w.area(), w.primative()));
        // )*

        let mut test = Vec::new();

        let center: Center = $center;
        let mut segments: Vec<Segment> = Vec::new();

        let viewport_width: usize = $vw;
        let viewport_height: usize = $vh;

        let mut total_width = 0;
        let mut total_height = 0;

        let mut max_width = 0;
        let mut max_height = 0;

        //The total height of largest widget in each segment.
        let mut total_height_of_largest = 0;
        let mut total_width_of_largest = 0;

        // let horizontal_wrap = 0;
        // let vertical_wrap = 0;

        const COUNT: usize = const { count_expr!($($widget),*) };
        // let seg = [const { Segment::new() }; COUNT];
        let mut i = 0;

        let mut widget_count = 0;

        //The first loop is required to calculate the segments.
        $(
            let area = $widget.area();

            i += 1;
            widget_count += 1;

            total_width += area.width;
            total_height += area.height;

            if area.width > max_width {
                max_width = area.width;
            }

            if area.height > max_height {
                max_height = area.height;
            }

            match center {
                Center::Horizontal => {
                    if (total_width + area.width > viewport_width) || i == COUNT {
                        segments.push(Segment {
                            direction: Direction::Horizontal,
                            size: total_width,
                            max: max_width,
                            widget_count,
                        });

                        total_height_of_largest += max_height;

                        max_height = 0;
                        max_width = 0;

                        total_width = 0;
                        widget_count = 0;
                    }
                }
                Center::Vertical => {
                    if (total_height + area.height > viewport_height) || i == COUNT  {
                        segments.push(Segment {
                            direction: Direction::Vertical,
                            size: total_height,
                            max: max_height,
                            widget_count,
                        });

                        total_width_of_largest += max_width;

                        max_height = 0;
                        max_width = 0;

                        total_height = 0;
                        widget_count = 0;
                    }
                },
                Center::Both => {
                }
            };
        )*

        let vspacing = viewport_height.saturating_sub(total_height_of_largest) / segments.len();
        let hspacing = viewport_width.saturating_sub(total_width_of_largest) / segments.len();
        // dbg!(vspacing, hspacing, viewport_width, total_width_of_largest, total_height_of_largest, &segments);

        let (mut x, mut y, mut spacing) = match center {
            Center::Horizontal => {
                let spacing = viewport_width.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                (spacing, 0, spacing)
            },
            Center::Vertical => {
                let spacing = viewport_height.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                (0, spacing, spacing)
            },
            Center::Both => todo!(),
        };

        let mut widget_index = 0;
        let mut segment_index = 0;

        $(
            let mut segment = &segments[segment_index];

            if widget_index >= segment.widget_count {
                widget_index = 0;
                segment_index += 1;
                segment = &segments[segment_index];

                match center {
                    Center::Horizontal => {
                        spacing = viewport_width.saturating_sub(segment.size) / (segment.widget_count + 1);
                        x = spacing;
                        y += segment.max + vspacing;
                    },
                    Center::Vertical => {
                        spacing = viewport_height.saturating_sub(segment.size) / (segment.widget_count + 1);
                        x += segment.max + hspacing;
                        y = spacing;
                    },
                    Center::Both => todo!(),
                };
            }

            let w = widget(&mut $widget);

            let area = w.area_mut();

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            // w.try_click(area);

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));

            match center {
                Center::Horizontal => x += spacing + area.width,
                Center::Vertical => y += spacing + area.height,
                Center::Both => todo!(),
            };

            widget_index += 1;
        )*

        test
    }};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flex {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn flex_xy(
    start: Flex,
    viewport_width: usize,
    viewport_height: usize,
    x: usize,
    y: usize,
) -> (usize, usize) {
    match start {
        Flex::TopLeft => (x, y),
        Flex::TopRight => (viewport_width - x, y),
        Flex::BottomLeft => (x, viewport_height - y),
        Flex::BottomRight => (viewport_width - x, viewport_height - y),
    }
}

#[macro_export]
macro_rules! flex {
    ($flex:expr, $direction:expr, $vw:expr, $vh:expr, $($widget:expr),*) => {{
        let mut test = Vec::new();

        let viewport_width: usize = $vw;
        let viewport_height: usize = $vh;

        let flex: Flex = $flex;
        let direction: Direction = $direction;

        let _x = 0;
        let _y = 0;
        let (mut x, mut y) = flex_xy(flex, viewport_width, viewport_height, _x, _y);
        let start_x = x;
        let start_y = y;

        let mut max_height = 0;
        let mut max_width = 0;

        $(
            let w = widget(&mut $widget);
            let area = w.area_mut().unwrap();

            match direction {
                Direction::Horizontal => {
                    if match flex {
                        Flex::TopLeft => (x + area.width) >= viewport_width,
                        Flex::TopRight => x.checked_sub(area.width).is_none(),
                        _ => false,
                    } {
                        x = start_x;
                        y += max_height;
                        max_height = 0;
                    }

                    if match flex {
                        Flex::BottomLeft => (x + area.width) >= viewport_width,
                        Flex::BottomRight => x.checked_sub(area.width).is_none(),
                        _ => false,
                    } {
                        x = start_x;
                        y -= max_height;
                        max_height = 0;
                    }
                }
                Direction::Vertical => {
                    if match flex {
                        Flex::TopLeft => (y + area.height) >= viewport_height,
                        Flex::BottomLeft => y.checked_sub(area.height).is_none(),
                        _ => false,
                    } {
                        y = start_y;
                        x += max_width;
                        max_width = 0;
                    }

                    if match flex {
                        Flex::TopRight => (y + area.height) >= viewport_height,
                        Flex::BottomRight => y.checked_sub(area.height).is_none(),
                        _ => false,
                    } {
                        y = start_y;
                        x -= max_width;
                        max_width = 0;
                    }
                }
            }

            if area.height > max_height {
                max_height = area.height;
            }

            if area.width > max_width {
                max_width = area.width;
            }

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.try_click();

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));

            match direction {
                Direction::Horizontal => {
                    match flex {
                        Flex::TopLeft | Flex::BottomLeft => x += area.width,
                        Flex::TopRight | Flex::BottomRight => x -= area.width,
                    };
                }
                Direction::Vertical =>  {
                    match flex {
                        Flex::TopLeft | Flex::TopRight => y += area.height,
                        Flex::BottomLeft | Flex::BottomRight => y -= area.height,
                    };
                }
            }
        )*

        test
    }};
}

#[derive(Debug)]
pub struct Segment {
    pub direction: Direction,
    ///Either the total height or width.
    ///Depends on the direction.
    pub size: usize,
    ///Max width or max height
    pub max: usize,
    pub widget_count: usize,
}
impl Segment {
    pub const fn new() -> Self {
        Self {
            direction: Direction::Horizontal,
            size: 0,
            max: 0,
            widget_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn flex_horizontal() {
        let mut h1 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut h2 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut h3 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        //viewport width and height.
        let vw = 50;
        let vh = 40;

        let test = flex!(Flex::TopLeft, Direction::Horizontal, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[1].0.y, 0);

        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[2].0.y, 20);

        let test = flex!(Flex::TopRight, Direction::Horizontal, vw, vh, h1, h2, h3);
        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 30);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[2].0.x, 50);
        assert_eq!(test[2].0.y, 20);

        let test = flex!(Flex::BottomLeft, Direction::Horizontal, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[2].0.y, 20);

        let test = flex!(Flex::BottomRight, Direction::Horizontal, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[1].0.x, 30);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[2].0.x, 50);
        assert_eq!(test[2].0.y, 20);

        let vw = 70;
        let vh = 40;

        let test = flex_center!(Center::Horizontal, vw, vh, h1, h2);
        assert_eq!(test.len(), 2);

        assert_eq!(test[0].0.x, 10);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 40);
        assert_eq!(test[1].0.y, 0);
    }

    #[test]
    fn flex_vertical() {
        let mut h1 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut h2 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut h3 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        //viewport width and height.
        let vw = 50;
        let vh = 50;

        let test = flex!(Flex::TopLeft, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 20);

        assert_eq!(test[2].0.x, 20);
        assert_eq!(test[2].0.y, 0);

        let test = flex!(Flex::TopRight, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 50);
        assert_eq!(test[1].0.y, 20);

        assert_eq!(test[2].0.x, 30);
        assert_eq!(test[2].0.y, 0);

        let test = flex!(Flex::BottomLeft, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 50);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 30);

        assert_eq!(test[2].0.x, 20);
        assert_eq!(test[2].0.y, 50);

        let test = flex!(Flex::BottomRight, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 50);

        assert_eq!(test[1].0.x, 50);
        assert_eq!(test[1].0.y, 30);

        assert_eq!(test[2].0.x, 30);
        assert_eq!(test[2].0.y, 50);
    }

    #[test]
    fn hcenter() {
        let vw = 40;
        let vh = 40;

        let mut h1 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut h2 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let test = flex_center!(Center::Horizontal, vw, vh, h1, h2);
        assert_eq!(test.len(), 2);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);

        let mut header3 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let test = flex_center!(Center::Horizontal, vw, vh, h1, h2, header3);
        assert_eq!(test.len(), 3);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);
        //Middle is (40 / 2) - ((40 / 2) / 2) = 10
        assert_eq!(test[2].0.x, 10);

        let mut header4 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let test = flex_center!(Center::Horizontal, vw, vh, h1, h2, header3, header4);

        assert_eq!(test.len(), 4);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[3].0.x, 20);
    }

    #[test]
    fn vcenter() {
        let vw = 40;
        let vh = 40;

        let mut h1 = Header {
            title: "hi",
            area: Rect { x: 0, y: 0, width: 20, height: 20 },
        };

        let mut h2 = Header {
            title: "hi",
            area: Rect { x: 0, y: 0, width: 20, height: 20 },
        };

        let mut h3 = Header {
            title: "hi",
            area: Rect { x: 0, y: 0, width: 20, height: 20 },
        };

        let test = flex_center!(Center::Vertical, vw, vh, h1, h2, h3);
        assert_eq!(test.len(), 3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 20);

        assert_eq!(test[2].0.x, 20);
        assert_eq!(test[2].0.y, 10);

        let vw = 40;
        let vh = 70;

        let test = flex_center!(Center::Vertical, vw, vh, h1, h2);
        assert_eq!(test.len(), 2);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 10);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 40);
    }
}
