use crate::prelude::*;

const HEIGHT: f32 = -10.0;

#[derive(Clone, PartialEq, Props)]
pub struct DimensionProperties {
    pub points: Vec<Point>,
    pub class: Option<String>,
    pub y_scale: f32,
}

pub struct DimensionData {
    pub start: Point,
    pub end: Point,
    pub start_top: Point,
    pub end_top: Point,
    pub center_top: Point,
    pub label: String,
}

#[component]
pub fn DimensionChart(props: DimensionProperties) -> Element {
    let dimensions = props.points.windows(2).map(|pair| {
        let start = pair.first().expect("should have two values");
        let end = pair.get(1).expect("should have two values");
        let top = if start.y > end.y {
            start.y - HEIGHT
        } else {
            end.y - HEIGHT
        };
        let diff = (end.y - start.y) / props.y_scale;
        DimensionData {
            start: *start,
            end: *end,
            start_top: Point::new(start.x, top),
            end_top: Point::new(end.x, top),
            center_top: Point::new((start.x + end.x) * 0.5, top),
            label: format!("{diff:.1}"),
        }
    });
    rsx! {
        g {
            class: "dim",
            for dim in dimensions {
                g {
                    line {
                        x1: dim.start.get_x(),
                        y1: dim.start.get_y(),
                        x2: dim.start_top.get_x(),
                        y2: dim.start_top.get_y(),
                    }
                    line {
                        x1: dim.end.get_x(),
                        y1: dim.end.get_y(),
                        x2: dim.end_top.get_x(),
                        y2: dim.end_top.get_y(),
                    }
                    line {
                        x1: dim.start_top.get_x(),
                        y1: dim.start_top.get_y(),
                        x2: dim.end_top.get_x(),
                        y2: dim.end_top.get_y(),
                    }
                    text {
                        x: dim.center_top.get_x(),
                        y: dim.center_top.get_y(),
                        "{dim.label}"
                    }
                }
            }
        }
    }
}
