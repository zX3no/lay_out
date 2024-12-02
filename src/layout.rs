use crate::*;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Center {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Quadrant {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexMode {
    Standard(Direction, Quadrant),
    Center(Center),
}

impl Default for FlexMode {
    fn default() -> Self {
        Self::Standard(Direction::Horizontal, Quadrant::TopLeft)
    }
}

#[derive(Debug)]
pub struct Segment {
    pub direction: Direction,
    ///Either the total height or width.
    ///Depends on the direction.
    pub size: usize,
    ///Max width or max height depends on direction.
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

pub struct FlexImpl<F> {
    pub f: Option<F>,
}

pub trait DrawFlex {
    fn call(&mut self, layout: &mut Flex<Self>)
    where
        Self: Sized;
}

impl<F> DrawFlex for FlexImpl<F>
where
    //I changed this to be FnMut for debug purposes. I don't know what issues this might cause.
    F: FnMut(FlexMode, usize, usize, usize, usize, usize, usize) -> Vec<(Rect, Primative)>,
{
    fn call(&mut self, flex: &mut Flex<Self>) {
        if let Some(ref mut f) = self.f {
            flex.debug = (f)(
                flex.mode,
                flex.viewport_width,
                flex.viewport_height,
                flex.area.x,
                flex.area.y,
                flex.margin,
                flex.padding,
            );
        }
    }
}

pub struct Flex<F: DrawFlex> {
    pub f: Option<F>,
    pub mode: FlexMode,
    pub area: Rect,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub debug: Vec<(Rect, Primative)>,
}

impl<F: DrawFlex> Flex<F> {
    //TODO: Not sure how to do this better.
    pub fn force_draw(&mut self) {
        let mut f = self.f.take();
        if let Some(f) = &mut f {
            f.call(self);
        }
        self.f = f;
    }
}

impl<F: DrawFlex> Drop for Flex<F> {
    fn drop(&mut self) {
        if let Some(mut f) = self.f.take() {
            f.call(self);
        }
    }
}

impl<F: DrawFlex> std::fmt::Debug for Flex<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flex")
            .field("f", if self.f.is_some() { &"Some" } else { &"None" })
            .field("flex", &self.mode)
            .field("area", &self.area)
            .field("padding", &self.padding)
            .field("margin", &self.margin)
            .field("viewport_width", &self.viewport_width)
            .field("viewport_height", &self.viewport_height)
            .field("debug", &self.debug)
            .finish()
    }
}

pub fn calculate_segments<T: Widget>(
    center: Center,
    segments: &mut Vec<Segment>,
    widget: &mut T,
    count: usize,
    viewport_width: usize,
    viewport_height: usize,
    mut i: usize,
    mut widget_count: usize,
    mut max_width: usize,
    mut max_height: usize,
    mut total_width: usize,
    mut total_height: usize,
    mut total_width_of_largest: usize,
    mut total_height_of_largest: usize,
) -> (usize, usize, usize, usize, usize, usize, usize, usize) {
    let area = widget.area();

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
            if (total_width + area.width > viewport_width) || i == count {
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
            if (total_height + area.height > viewport_height) || i == count {
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
        }
        Center::Both => {
            todo!()
        }
    };

    (
        i,
        widget_count,
        max_width,
        max_height,
        total_width,
        total_height,
        total_width_of_largest,
        total_height_of_largest,
    )
}

pub fn draw_segments<T: Widget>(
    widget: &mut T,
    center: Center,
    segments: &[Segment],
    mut widget_index: usize,
    mut segment_index: usize,
    mut x: usize,
    mut y: usize,
    viewport_width: usize,
    viewport_height: usize,
    mut spacing: usize,
    vspacing: usize,
    hspacing: usize,
    temp: &mut Vec<(Rect, Primative)>,
) -> (usize, usize, usize, usize, usize) {
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
            }
            Center::Vertical => {
                spacing = viewport_height.saturating_sub(segment.size) / (segment.widget_count + 1);
                x += segment.max + hspacing;
                y = spacing;
            }
            Center::Both => todo!(),
        };
    }

    let area = widget.area_mut().unwrap();

    area.x = x;
    area.y = y;

    //Stop the mutable borrow.
    let area = widget.area();

    //Click the widget once the layout is calculated.
    widget.try_click();

    //This is where the draw call would typically be issued.
    temp.push((area, widget.primative()));

    match center {
        Center::Horizontal => x += spacing + area.width,
        Center::Vertical => y += spacing + area.height,
        Center::Both => todo!(),
    };

    widget_index += 1;

    (widget_index, segment_index, x, y, spacing)
}

pub fn flex_xy(
    start: Quadrant,
    viewport_width: usize,
    viewport_height: usize,
    x: usize,
    y: usize,
) -> (usize, usize) {
    match start {
        Quadrant::TopLeft => (x, y),
        Quadrant::TopRight => (viewport_width - x, y),
        Quadrant::BottomLeft => (x, viewport_height - y),
        Quadrant::BottomRight => (viewport_width - x, viewport_height - y),
    }
}

pub fn flex_standard<T: Widget>(
    direction: Direction,
    quadrant: Quadrant,
    widget: &mut T,
    mut x: usize,
    mut y: usize,
    start_x: usize,
    start_y: usize,
    mut max_width: usize,
    mut max_height: usize,
    viewport_width: usize,
    viewport_height: usize,
    temp: &mut Vec<(Rect, Primative)>,
) -> (usize, usize, usize, usize) {
    let area = widget.area_mut().unwrap();

    match direction {
        Direction::Horizontal => {
            if match quadrant {
                Quadrant::TopLeft => (x + area.width) >= viewport_width,
                Quadrant::TopRight => x.checked_sub(area.width).is_none(),
                _ => false,
            } {
                x = start_x;
                y += max_height;
                max_height = 0;
            }

            if match quadrant {
                Quadrant::BottomLeft => (x + area.width) >= viewport_width,
                Quadrant::BottomRight => x.checked_sub(area.width).is_none(),
                _ => false,
            } {
                x = start_x;
                y -= max_height;
                max_height = 0;
            }
        }
        Direction::Vertical => {
            if match quadrant {
                Quadrant::TopLeft => (y + area.height) >= viewport_height,
                Quadrant::BottomLeft => y.checked_sub(area.height).is_none(),
                _ => false,
            } {
                y = start_y;
                x += max_width;
                max_width = 0;
            }

            if match quadrant {
                Quadrant::TopRight => (y + area.height) >= viewport_height,
                Quadrant::BottomRight => y.checked_sub(area.height).is_none(),
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
    let area = widget.area();

    //Click the widget once the layout is calculated.
    widget.try_click();

    //This is where the draw call would typically be issued.
    temp.push((area, widget.primative()));

    match direction {
        Direction::Horizontal => {
            match quadrant {
                Quadrant::TopLeft | Quadrant::BottomLeft => x += area.width,
                Quadrant::TopRight | Quadrant::BottomRight => x -= area.width,
            };
        }
        Direction::Vertical => {
            match quadrant {
                Quadrant::TopLeft | Quadrant::TopRight => y += area.height,
                Quadrant::BottomLeft | Quadrant::BottomRight => y -= area.height,
            };
        }
    }

    (x, y, max_width, max_height)
}

#[macro_export]
macro_rules! flex {
    ($($widget:expr),*) => {{
        let f = |flex: FlexMode, viewport_width: usize, viewport_height: usize, x: usize, y: usize, _margin: usize, _padding: usize| -> Vec<(Rect, Primative)> {
            let mut temp = Vec::new();

            match flex {
                FlexMode::Standard(direction, quadrant) => {
                    //Could pack all of this into a struct. It might be faster.
                    //Might check later.
                    let (x, y) = flex_xy(quadrant, viewport_width, viewport_height, x, y);
                    let start_x = x;
                    let start_y = y;
                    let max_height = 0;
                    let max_width = 0;

                    $(
                        let w = widget(&mut $widget);
                        #[allow(unused)]
                        let (x, y, max_width, max_height) = flex_standard(direction,quadrant, w, x, y, start_x, start_y, max_height, max_width, viewport_width, viewport_height, &mut temp);
                    )*
                }
                FlexMode::Center(center) => {
                    let mut segments: Vec<Segment> = Vec::new();

                    let total_width = 0;
                    let total_height = 0;
                    let max_width = 0;
                    let max_height = 0;
                    //The total height of largest widget in each segment.
                    let total_height_of_largest = 0;
                    let total_width_of_largest = 0;
                    let i = 0;
                    let widget_count = 0;

                    const COUNT: usize = const { count_expr!($($widget),*) };

                    //The first loop is required to calculate the segments.
                    $(
                        let w = widget(&mut $widget);

                        #[allow(unused)]
                        let (i, widget_count, max_width, max_height, total_width, total_height, total_width_of_largest, total_height_of_largest) = calculate_segments(
                            center,
                            &mut segments,
                            w,
                            COUNT,
                            viewport_width,
                            viewport_height,
                            i,
                            widget_count,
                            max_width,
                            max_height,
                            total_width,
                            total_height,
                            total_width_of_largest,
                            total_height_of_largest,
                        );
                    )*

                    //This is named poorly, I honestly can't even remember what this is for...
                    let vspacing = viewport_height.saturating_sub(total_height_of_largest) / segments.len();
                    let hspacing = viewport_width.saturating_sub(total_width_of_largest) / segments.len();

                    //Spacing is really the segment spacing.
                    let (x, y, spacing) = match center {
                        Center::Horizontal => {
                            let spacing = viewport_width.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                            (spacing, 0, spacing)
                        },
                        Center::Vertical => {
                            let spacing = viewport_height.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                            (0, spacing, spacing)
                        },
                        Center::Both => {
                            let x = viewport_width.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                            let y = viewport_height.saturating_sub(segments[0].size) / (segments[0].widget_count + 1);
                            //I think I'll need to keep both types of segment spacing.
                            (x, y, 0)
                        },
                    };

                    let widget_index = 0;
                    let segment_index = 0;

                    $(
                        let w = widget(&mut $widget);
                        #[allow(unused)]
                        let (widget_index, segment_index, x, y, spacing) = draw_segments(
                            w,
                            center,
                            &segments,
                            widget_index,
                            segment_index,
                            x,
                            y,
                            viewport_width,
                            viewport_height,
                            spacing,
                            vspacing,
                            hspacing,
                            &mut temp,
                        );
                    )*
                }
            }

            temp
        };

        $crate::Flex {
            f: Some(FlexImpl { f: Some(f) }),
            mode: FlexMode::default(),
            area: Rect::default(),
            padding: 0,
            margin: 0,
            viewport_width: 0,
            viewport_height: 0,
            debug: Vec::new(),
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn flex_horizontal_new() {
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

        let mut flex = flex!(h1, h2, h3);
        flex.viewport_width = 50;
        flex.viewport_height = 40;
        flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::TopLeft);
        flex.force_draw();

        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[0].0.y, 0);

        assert_eq!(flex.debug[1].0.x, 20);
        assert_eq!(flex.debug[1].0.y, 0);

        assert_eq!(flex.debug[2].0.x, 0);
        assert_eq!(flex.debug[2].0.y, 20);

        flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::TopRight);
        flex.force_draw();

        assert_eq!(flex.debug[0].0.x, 50);
        assert_eq!(flex.debug[0].0.y, 0);

        assert_eq!(flex.debug[1].0.x, 30);
        assert_eq!(flex.debug[0].0.y, 0);

        assert_eq!(flex.debug[2].0.x, 50);
        assert_eq!(flex.debug[2].0.y, 20);

        flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::BottomLeft);
        flex.force_draw();

        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[0].0.y, 40);

        assert_eq!(flex.debug[1].0.x, 20);
        assert_eq!(flex.debug[0].0.y, 40);

        assert_eq!(flex.debug[2].0.x, 0);
        assert_eq!(flex.debug[2].0.y, 20);

        flex.mode = FlexMode::Standard(Direction::Horizontal, Quadrant::BottomRight);
        flex.force_draw();

        assert_eq!(flex.debug[0].0.x, 50);
        assert_eq!(flex.debug[0].0.y, 40);

        assert_eq!(flex.debug[1].0.x, 30);
        assert_eq!(flex.debug[0].0.y, 40);

        assert_eq!(flex.debug[2].0.x, 50);
        assert_eq!(flex.debug[2].0.y, 20);
    }

    #[test]
    fn flex_center_new() {
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

        let mut h4 = Header {
            title: "hi",
            area: Rect {
                x: 0,
                y: 0,
                width: 20,
                height: 20,
            },
        };

        let mut flex = flex!(h1, h2);
        flex.viewport_width = 40;
        flex.viewport_height = 40;
        flex.mode = FlexMode::Center(Center::Horizontal);
        flex.force_draw();

        assert_eq!(flex.debug.len(), 2);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);

        drop(flex);

        let mut flex = flex!(h1, h2, h3);
        flex.viewport_width = 40;
        flex.viewport_height = 40;
        flex.mode = FlexMode::Center(Center::Horizontal);
        flex.force_draw();

        assert_eq!(flex.debug.len(), 3);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);
        //Middle is (40 / 2) - ((40 / 2) / 2) = 10
        assert_eq!(flex.debug[2].0.x, 10);

        drop(flex);

        let mut flex = flex!(h1, h2, h3, h4);
        flex.viewport_width = 40;
        flex.viewport_height = 40;
        flex.mode = FlexMode::Center(Center::Horizontal);
        flex.force_draw();

        assert_eq!(flex.debug.len(), 4);
        assert_eq!(flex.debug[0].0.x, 0);
        assert_eq!(flex.debug[1].0.x, 20);
        assert_eq!(flex.debug[2].0.x, 0);
        assert_eq!(flex.debug[3].0.x, 20);
    }

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

        let test = flex_standard!(Quadrant::TopLeft, Direction::Horizontal, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[1].0.y, 0);

        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[2].0.y, 20);

        let test = flex_standard!(
            Quadrant::TopRight,
            Direction::Horizontal,
            vw,
            vh,
            h1,
            h2,
            h3
        );
        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 30);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[2].0.x, 50);
        assert_eq!(test[2].0.y, 20);

        let test = flex_standard!(
            Quadrant::BottomLeft,
            Direction::Horizontal,
            vw,
            vh,
            h1,
            h2,
            h3
        );

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[1].0.x, 20);
        assert_eq!(test[0].0.y, 40);

        assert_eq!(test[2].0.x, 0);
        assert_eq!(test[2].0.y, 20);

        let test = flex_standard!(
            Quadrant::BottomRight,
            Direction::Horizontal,
            vw,
            vh,
            h1,
            h2,
            h3
        );

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

        let test = flex_standard!(Quadrant::TopLeft, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 20);

        assert_eq!(test[2].0.x, 20);
        assert_eq!(test[2].0.y, 0);

        let test = flex_standard!(Quadrant::TopRight, Direction::Vertical, vw, vh, h1, h2, h3);

        assert_eq!(test[0].0.x, 50);
        assert_eq!(test[0].0.y, 0);

        assert_eq!(test[1].0.x, 50);
        assert_eq!(test[1].0.y, 20);

        assert_eq!(test[2].0.x, 30);
        assert_eq!(test[2].0.y, 0);

        let test = flex_standard!(
            Quadrant::BottomLeft,
            Direction::Vertical,
            vw,
            vh,
            h1,
            h2,
            h3
        );

        assert_eq!(test[0].0.x, 0);
        assert_eq!(test[0].0.y, 50);

        assert_eq!(test[1].0.x, 0);
        assert_eq!(test[1].0.y, 30);

        assert_eq!(test[2].0.x, 20);
        assert_eq!(test[2].0.y, 50);

        let test = flex_standard!(
            Quadrant::BottomRight,
            Direction::Vertical,
            vw,
            vh,
            h1,
            h2,
            h3
        );

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
