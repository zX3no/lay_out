#![feature(associated_type_defaults)]

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
        Click { widget: self, click: (button, f) }
    }

    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
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
    fn area_mut(&mut self) -> &mut Rect;
    fn primative(&self) -> Primative;
    //TODO: The area might go unused with the new macro and could be removed.
    fn click(&mut self, area: Rect) {}
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

pub static mut VIEWPORT_WIDTH: usize = 800;
pub static mut VIEWPORT_HEIGHT: usize = 600;

//Does not take in references.
#[macro_export]
macro_rules! tlayout {
    (($widget:expr),*) => {};
}

//This does centers all widgets horizontally.
#[macro_export]
macro_rules! layout {
    ($($widget:expr),*) => {{
        // let mut widgets = Vec::new();

        // $(
        //     let w = widget(&mut $widget);
        //     widgets.push((w.area(), w.primative()));
        // )*

        let mut test = Vec::new();

        let mut segments: Vec<Segment> = Vec::new();
        let viewport_width: usize = unsafe {VIEWPORT_WIDTH};
        let viewport_height: usize = unsafe {VIEWPORT_HEIGHT};
        let mut total_width = 0;
        let mut max_width = 0;

        //The total height of largest widget in each segment.
        let mut total_height_of_largest = 0;
        let mut total_hsegments = 0;

        let mut max_height = 0;
        let horizontal_wrap = 0;
        let vertical_wrap = 0;

        // let count = widgets.len();
        let count = count_expr!($($widget),*);
        let mut i = 0;

        let mut widget_count = 0;

        $(
            let area = $widget.area();

            i += 1;

            //Skip the zero width segment.
            //This is pretty much a hack and should be removed in the third re-write.
            if total_width + area.width > viewport_width && !(total_width == 0 || max_width == 0) {
                segments.push(Segment {
                    direction: Direction::Horizontal,
                    size: total_width,
                    max: max_width,
                    widget_count,
                });

                total_hsegments += 1;
                total_height_of_largest += max_height;
                max_height = 0;
                total_width = 0;
                widget_count = 0;
                max_width = 0;
            }

            total_width += area.width;
            // total_height += area.height;

            if area.width > max_width {
                max_width = area.width;
            }

            if area.height > max_height {
                max_height = area.height;
            }

            //TODO: Could just have a start and end index into widgets
            //This would be faster and less stupid.
            // segment_widgets.push((area, primative));
            widget_count += 1;

            //Don't like this part.
            if (i == count) {
                total_hsegments += 1;
                total_height_of_largest += max_height;
                segments.push(Segment {
                    direction: Direction::Horizontal,
                    size: total_width,
                    max: max_width,
                    widget_count,
                });
                widget_count = 0;
            }
        )*

        // dbg!(&segments);
        let vspacing =
            viewport_height.saturating_sub(total_height_of_largest) / (total_hsegments + 1);
        let mut x = 0;
        let mut y = vspacing;
        let mut wid = 0;
        let mut seg = 0;
        let mut spacing = 0;
        let mut max_height = 0;

        $(
            let mut segment = &segments[seg];
            if wid == 0 {
                spacing = viewport_width.saturating_sub(segment.size) / (segment.widget_count + 1);
                x = spacing;
            }

            if wid >= segment.widget_count {
                wid = 0;
                seg += 1;
                y += max_height + vspacing;

                segment = &segments[seg];
                spacing = viewport_width.saturating_sub(segment.size) / (segment.widget_count + 1);
                x = spacing;
            }

            let w = widget(&mut $widget);

            let area = w.area_mut();

            if area.height > max_height {
                max_height = area.height;
            }

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.click(area);

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));
            x += spacing + area.width;

            wid += 1;
        )*

        test
    }};
}

#[macro_export]
macro_rules! flex_top_left {
    ($($widget:expr),*) => {{
        let mut test = Vec::new();

        let mut viewport_width = unsafe { VIEWPORT_WIDTH };
        let mut viewport_height = unsafe { VIEWPORT_HEIGHT };

        //User should be able to configure these.
        let start_x = 0;
        let start_y = 0;
        let (mut x, mut y) = flex_xy(Flex::TopLeft, start_x, start_y);

        let mut largest_widget = 0;

        $(
            let w = widget(&mut $widget);
            let area = w.area_mut();

            //Widget is too wide
            if (x + area.width) >= viewport_width {
                x = start_x;
                y += largest_widget;
                largest_widget = 0;
            }

            if area.height > largest_widget {
                largest_widget = area.height;
            }

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.click(area);

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));

            x += area.width;
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

pub const fn flex_xy(start: Flex, x: usize, y: usize) -> (usize, usize) {
    let viewport_width = unsafe { VIEWPORT_WIDTH };
    let viewport_height = unsafe { VIEWPORT_HEIGHT };

    match start {
        Flex::TopLeft => (x, y),
        Flex::TopRight => (viewport_width - x, y),
        Flex::BottomLeft => (x, viewport_height - y),
        Flex::BottomRight => (viewport_width - x, viewport_height - y),
    }
}

#[macro_export]
macro_rules! flex {
    ($flex:expr, $($widget:expr),*) => {{
        let mut test = Vec::new();

        let mut viewport_width = unsafe { VIEWPORT_WIDTH };
        let mut viewport_height = unsafe { VIEWPORT_HEIGHT };

        let flex: Flex = $flex;

        let (mut x, mut y) = flex_xy(flex, 0, 0);
        let mut start_x = x;
        let mut start_y = y;

        let mut largest_widget = 0;

        $(
            let w = widget(&mut $widget);
            let area = w.area_mut();

            if match flex {
                Flex::TopLeft => (x + area.width) >= viewport_width,
                Flex::TopRight => x.checked_sub(area.width).is_none(),
                Flex::BottomLeft => todo!(),
                Flex::BottomRight => todo!(),
            } {
                x = start_x;
                y += largest_widget;
                largest_widget = 0;
            }

            // if flex == Flex::TopLeft && (x + area.width) >= viewport_width{
            //     x = start_x;
            //     y += largest_widget;
            //     largest_widget = 0;
            // }

            // if flex == Flex::TopRight && x.checked_sub(area.width).is_none() {
            //     x = start_x;
            //     y += largest_widget;
            //     largest_widget = 0;
            // }

            if area.height > largest_widget {
                largest_widget = area.height;
            }

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.click(area);

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));

            match flex {
                Flex::TopLeft => x += area.width,
                Flex::TopRight => x -= area.width,
                Flex::BottomLeft => todo!(),
                Flex::BottomRight => todo!(),
            };
        )*

        test
    }};
}

// let temp = flex(10, 10, &widgets);
//Not sure how to call click here.
//There is a new area that is created and not set to the original widget despite
//being passed in with a mutable reference.
//That would mean you would need to call an on click function that takes in
//the original widget `&mut $widget` and a new calculated area. I think
//currently the area is gotten from the widget itself with widget.area().
//```rs
//if clicked(ctx, &mut self.widget, self.click.$idx.0) {
//    self.click.$idx.1(&mut self.widget);
//}
//```
//clicked takes in &mut self.widget then calls `let area = *self.area_mut().unwrap();`
//To fix this I guess I'd need change try_click to be, fn try_click(&mut self, area: Rect);
//Then
//```rs
//if clicked(ctx, area, self.click.$idx.0) {
//    self.click.$idx.1(&mut self.widget);
//}
//```
//do all the layout calculations and figue out where everything should go.

//I'm not sure if it's possible to calculate the widgets with a single step of the macro.
//If we assume that everything that is passed in can be a reference to something. that should make copy pasting easier.

//once the widget has the correct position, run the click function.

#[derive(Debug)]
pub struct Segment {
    pub direction: Direction,
    ///Either the total height or width.
    ///Depends on the direction.
    pub size: usize,
    ///Max width or max height
    pub max: usize,
    // pub widgets: Vec<(Rect, Primative)>,
    pub widget_count: usize,
}

#[derive(Debug)]
pub struct OldSegment {
    pub direction: Direction,
    ///Either the total height or width.
    ///Depends on the direction.
    pub size: usize,
    ///Max width or max height
    pub max: usize,
    pub widgets: Vec<(Rect, Primative)>,
}

pub fn flex(viewport_width: usize, viewport_height: usize, widgets: &[(Rect, Primative)]) -> Vec<(Rect, Primative)> {
    let mut temp_primatives = Vec::new();
    let mut segments: Vec<OldSegment> = Vec::new();
    // let viewport_width = ctx().area.width;
    // let viewport_height = ctx().area.height;
    let mut total_width = 0;
    let mut max_width = 0;

    //The total height of largest widget in each segment.
    let mut total_height_of_largest = 0;
    let mut total_hsegments = 0;

    let mut max_height = 0;
    let horizontal_wrap = 0;
    let vertical_wrap = 0;

    let mut segment_widgets = Vec::new();
    let count = widgets.len();
    let mut i = 0;

    for (area, primative) in widgets {
        i += 1;

        //Skip the zero width segment.
        //This is pretty much a hack and should be removed in the third re-write.
        if total_width + area.width > viewport_width && !(total_width == 0 || max_width == 0) {
            segments.push(OldSegment {
                direction: Direction::Horizontal,
                size: total_width,
                max: max_width,
                widgets: core::mem::take(&mut segment_widgets),
            });

            total_hsegments += 1;
            total_height_of_largest += max_height;
            max_height = 0;
            total_width = 0;
            max_width = 0;
        }

        total_width += area.width;
        // total_height += area.height;

        if area.width > max_width {
            max_width = area.width;
        }

        if area.height > max_height {
            max_height = area.height;
        }

        //TODO: Could just have a start and end index into widgets
        //This would be faster and less stupid.
        segment_widgets.push((*area, primative.clone()));

        //Don't like this part.
        if i == count {
            total_hsegments += 1;
            total_height_of_largest += max_height;
            segments.push(OldSegment {
                direction: Direction::Horizontal,
                size: total_width,
                max: max_width,
                widgets: core::mem::take(&mut segment_widgets),
            })
        }
    }

    let vspacing = viewport_height.saturating_sub(total_height_of_largest) / (total_hsegments + 1);
    let mut y = vspacing;

    for segment in segments {
        let spacing = viewport_width.saturating_sub(segment.size) / (segment.widgets.len() + 1);
        let mut x = spacing;
        let mut max_height = 0;

        match segment.direction {
            Direction::Horizontal => {
                for (mut area, primative) in segment.widgets {
                    if area.height > max_height {
                        max_height = area.height;
                    }

                    area.x = x;
                    area.y = y;
                    // unsafe { COMMAND_QUEUE.push(Primative { area, primative }) };
                    temp_primatives.push((area, primative));
                    x += spacing + area.width;
                }
                y += max_height + vspacing;
            }
            Direction::Vertical => {}
        }
    }

    temp_primatives
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn hcenter() {
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
        assert_eq!(test.len(), 2);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);

        let mut header3 = Header {
            title: "hi",
            area: Rect { x: 0, y: 0, width: 20, height: 20 },
        };

        let test = layout!(header, header2, header3);
        assert_eq!(test.len(), 3);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);
        //Middle is (40 / 2) - ((40 / 2) / 2) = 10
        assert_eq!(test[2].0.x, 10);

        let mut header4 = Header {
            title: "hi",
            area: Rect { x: 0, y: 0, width: 20, height: 20 },
        };

        let test = layout!(header, header2, header3, header4);

        assert_eq!(test.len(), 4);
        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[3].0.x, 20);
    }
}
