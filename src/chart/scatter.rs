use crate::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScatterProperties {
    pub data: Vec<ScatterData>,
    pub class: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScatterData {
    pub point: Point,
    pub size: f32,
    pub class: Option<String>,
    pub descender: Option<Point>,
}

#[component]
pub fn ScatterChart(props: ScatterProperties) -> Element {
    rsx! {
        g {
            class: props.class.clone(),
            for data in props.data.iter() {
                g {
                    class: data.class.clone(),
                    if let Some(descender) = data.descender {
                        line {
                            x1: data.point.get_x(),
                            y1: data.point.get_y(),
                            x2: descender.get_x(),
                            y2: descender.get_y(),
                        }
                    }
                    circle
                    {
                        cx: data.point.get_x(),
                        cy: data.point.get_y(),
                        r: data.size,
                    }
                }
            }
        }
    }
}
