use crate::prelude::dioxus_elements::input_data::MouseButton;
use crate::prelude::*;

const MARGIN: f32 = 10.0;
const SVG_WIDTH: f32 = 705.0;
const ZOOM_MIN: usize = 1;
const ZOOM_MAX: usize = 100;

#[derive(Clone, Debug, Default)]
pub struct View {
    /// Size of the SVG element in screen units
    pub element_size: Point,
    /// Size of the chart in SVG units
    pub chart_size: Point,
    /// Center of the view in SVG units
    pub center: Point,
    /// Zoom level between 1 and 10
    pub zoom: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DragAction {
    pub from: Option<Point>,
}

impl View {
    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub fn new(range: &EntryRange) -> Self {
        let chart_size = range.get_total_days() as f32 + MARGIN * 2.0;
        let chart_size = Point::new(chart_size, chart_size);
        Self {
            chart_size,
            // TODO: Dynamically set SVG_WIDTH
            element_size: Point::new(SVG_WIDTH, SVG_WIDTH),
            center: chart_size * 0.5,
            zoom: ZOOM_MIN,
        }
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub(crate) fn get_zoom_scale(&self) -> f32 {
        let zoom = self.zoom as f32;
        let base: f32 = 1.0;
        let growth_factor: f32 = 0.9;
        let scale = base * growth_factor.powf(zoom - 1.0);
        debug!("Chart zoom: {zoom}");
        debug!("Chart scale: {scale:.2}");
        scale
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub(crate) fn get_min(&self) -> Point {
        self.center - self.chart_size * 0.5 * self.get_zoom_scale() - Point::new(MARGIN, MARGIN)
    }

    #[allow(clippy::as_conversions, clippy::cast_precision_loss)]
    pub(crate) fn get_viewbox(&self) -> String {
        let (min_x, min_y) = self.get_min().into();
        let (width, height) = (self.chart_size * self.get_zoom_scale()).into();
        format!("{min_x:.5} {min_y:.5} {width:.5} {height:.5}")
    }

    fn screen_to_svg(&self, point: Point) -> Point {
        let screen_to_svg = self.chart_size / self.element_size * self.get_zoom_scale();
        point * screen_to_svg
    }

    fn move_center_by_screen_delta(&mut self, delta: Point) {
        let delta = self.screen_to_svg(delta);
        self.center += delta;
    }

    pub(crate) fn on_wheel(event: Event<WheelData>, view: Signal<View>) {
        let delta = event.delta().strip_units().y;
        if delta < 0.0 {
            zoom_in(view);
        } else if delta > 0.0 {
            zoom_out(view);
        } else {
            debug!("No delta");
        }
    }

    pub fn on_mousedown(event: Event<MouseData>, mut action: Signal<DragAction>) {
        if event.data.trigger_button() != Some(MouseButton::Primary) {
            return;
        }
        event.prevent_default();
        let page = event.data.page_coordinates();
        action.set(DragAction {
            from: Some(page.into()),
        });
    }

    pub fn on_mousemove(
        event: Event<MouseData>,
        mut action: Signal<DragAction>,
        mut view: Signal<View>,
    ) {
        event.prevent_default();
        update_position(event, &mut action, &mut view);
    }

    pub fn on_mouseup(
        event: Event<MouseData>,
        mut action: Signal<DragAction>,
        mut view: Signal<View>,
    ) {
        if event.data.trigger_button() != Some(MouseButton::Primary) {
            return;
        }
        event.prevent_default();
        update_position(event, &mut action, &mut view);
        action.set(DragAction { from: None });
    }
}

fn zoom_in(mut signal: Signal<View>) {
    let zoom = signal.read().zoom;
    if zoom < ZOOM_MAX {
        debug!("Zooming in to {}", zoom + 1);
        signal.with_mut(|view| view.zoom += 1);
    } else {
        debug!("Reached max zoom level: {}", zoom);
    }
}

fn zoom_out(mut signal: Signal<View>) {
    let zoom = signal.read().zoom;
    if zoom > ZOOM_MIN {
        debug!("Zooming out to {}", zoom - 1);
        signal.with_mut(|view| view.zoom -= 1);
    } else {
        debug!("Reached min zoom level: {zoom}");
    }
}

fn update_position(
    event: Event<MouseData>,
    action: &mut Signal<DragAction>,
    view: &mut Signal<View>,
) {
    let Some(from) = action.read().from else {
        return;
    };
    let current: Point = event.data.page_coordinates().into();
    action.set(DragAction {
        from: Some(current),
    });
    let delta = from - current;
    debug!("Moving {delta}");
    view.with_mut(|view| view.move_center_by_screen_delta(delta));
}
