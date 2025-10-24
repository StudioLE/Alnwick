use crate::prelude::*;

#[component]
pub fn Chart() -> Element {
    let state = use_context::<EntryState>();
    let collection = state.get_cloned();
    let view = use_signal(|| View::new(&collection.range));
    let drag = use_signal(DragAction::default);
    let factory = ChartFactory::new(collection);
    let weight_scatter: Vec<_> = factory.get_weight_scatter();
    let trend: Vec<Point> = factory.get_trend_points();
    let sma1c_line = factory.get_points_for(|x| x.sma1c);
    rsx! {
        svg {
            onwheel: move |event| View::on_wheel(event, view),
            onmousedown: move |event| View::on_mousedown(event, drag),
            onmouseup: move |event| View::on_mouseup(event, drag, view),
            onmousemove: move |event| View::on_mousemove(event, drag, view),
            view_box: view.read().get_viewbox(),
            preserve_aspect_ratio: "xMidYMid slice",
            role: "img",
            Style {}
            DimensionChart {
                class: "trend",
                points: trend.clone(),
                y_scale: factory.y_scale,
            }
            LineChart {
                class: "trend",
                points: trend,
            }
            LineChart {
                class: "sma1c",
                points: sma1c_line,
            }
            ScatterChart {
                class: None,
                data: weight_scatter,
            }
        }
    }
}
