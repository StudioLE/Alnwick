use crate::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PolylineProperties {
    pub points: Vec<Point>,
    pub class: Option<String>,
}

#[component]
pub fn LineChart(props: PolylineProperties) -> Element {
    let points = props.points.iter().fold(String::new(), |mut acc, point| {
        acc.push_str(&point.get_x());
        acc.push(',');
        acc.push_str(&point.get_y());
        acc.push(' ');
        acc
    });
    rsx! {
        polyline {
            class: props.class.unwrap_or_default(),
            points,
        }
    }
}
