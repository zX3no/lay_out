#[macro_export]
macro_rules! flex_standard {
    ($flex:expr, $direction:expr, $vw:expr, $vh:expr, $($widget:expr),*) => {{
        let mut test = Vec::new();

        let viewport_width: usize = $vw;
        let viewport_height: usize = $vh;

        let flex: Quadrant = $flex;
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
                        Quadrant::TopLeft => (x + area.width) >= viewport_width,
                        Quadrant::TopRight => x.checked_sub(area.width).is_none(),
                        _ => false,
                    } {
                        x = start_x;
                        y += max_height;
                        max_height = 0;
                    }

                    if match flex {
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
                    if match flex {
                        Quadrant::TopLeft => (y + area.height) >= viewport_height,
                        Quadrant::BottomLeft => y.checked_sub(area.height).is_none(),
                        _ => false,
                    } {
                        y = start_y;
                        x += max_width;
                        max_width = 0;
                    }

                    if match flex {
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
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.try_click();

            //This is where the draw call would typically be issued.
            test.push((area, w.primative()));

            match direction {
                Direction::Horizontal => {
                    match flex {
                        Quadrant::TopLeft | Quadrant::BottomLeft => x += area.width,
                        Quadrant::TopRight | Quadrant::BottomRight => x -= area.width,
                    };
                }
                Direction::Vertical =>  {
                    match flex {
                        Quadrant::TopLeft | Quadrant::TopRight => y += area.height,
                        Quadrant::BottomLeft | Quadrant::BottomRight => y -= area.height,
                    };
                }
            }
        )*

        test
    }};
}

//This does centers all widgets horizontally.
#[macro_export]
macro_rules! flex_center {
    ($center:expr, $vw:expr, $vh:expr, $($widget:expr),*) => {{
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
                    todo!()
                }
            };
        )*

        //This is named poorly, I honestly can't even remember what this is for...
        let vspacing = viewport_height.saturating_sub(total_height_of_largest) / segments.len();
        let hspacing = viewport_width.saturating_sub(total_width_of_largest) / segments.len();
        // dbg!(vspacing, hspacing, viewport_width, total_width_of_largest, total_height_of_largest, &segments);

        //Spacing is really the segment spacing.
        let (mut x, mut y, mut spacing) = match center {
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

            let area = w.area_mut().unwrap();

            area.x = x;
            area.y = y;

            //Stop the mutable borrow.
            let area = w.area();

            //Click the widget once the layout is calculated.
            w.try_click();

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
