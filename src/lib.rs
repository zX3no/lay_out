#![feature(associated_type_defaults)]
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
    fn click(&mut self) {}
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
        let mut viewport_width: usize = unsafe {VIEWPORT_WIDTH};
        let mut viewport_height: usize = unsafe {VIEWPORT_HEIGHT};
        let mut total_width = 0;
        let mut max_width = 0;

        //The total height of largest widget in each segment.
        let mut total_height_of_largest = 0;
        let mut total_hsegments = 0;

        let mut max_height = 0;
        let mut horizontal_wrap = 0;
        let mut vertical_wrap = 0;

        // let count = widgets.len();
        let count = count_expr!($($widget),*);
        let mut i = 0;

        let mut widget_count = 0;

        $(
            let area = $widget.area();
            let primative = $widget.primative();

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

        let mut vspacing =
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

            if wid > segment.widget_count {
                wid = 0;
                seg += 1;
                y += max_height + vspacing;
                segment = &segments[seg];
                spacing = viewport_width.saturating_sub(segment.size) / (segment.widget_count + 1);
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
            w.click();

            test.push((area, w.primative()));
            x += spacing + area.width;

            wid += 1;
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
