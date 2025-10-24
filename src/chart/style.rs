use crate::prelude::*;

#[component]
pub fn Style() -> Element {
    rsx! {
        style {
"
circle {{ fill: {SHOT_NONE} }}
.shot circle {{ fill: {SHOT_UNKNOWN} }}
.d25 circle {{ fill: {SHOT_25} }}
.d50 circle {{ fill: {SHOT_50} }}
.d75 circle {{ fill: {SHOT_75} }}
.d100 circle {{ fill: {SHOT_100} }}
.d125 circle {{ fill: {SHOT_125} }}
.d150 circle {{ fill: {SHOT_150} }}
line {{ 
  stroke-width: 0.25;
  stroke: {SHOT_NONE}
}}
.dim line {{
  stroke-width: 0.1;
  stroke: {EMERALD_800};
}}
.shot line {{ stroke: {SHOT_UNKNOWN} }}
.d25 line {{ stroke: {SHOT_25} }}
.d50 line {{ stroke: {SHOT_50} }}
.d75 line {{ stroke: {SHOT_75} }}
.d100 line {{ stroke: {SHOT_100} }}
.d125 line {{ stroke: {SHOT_125} }}
.d150 line {{ stroke: {SHOT_150} }}
polyline {{
  fill: none;
  stroke: {SHOT_NONE};
  stroke-linejoin: bevel;
  stroke-width: 0.75;
}}
polyline.sma1c {{
  stroke: {SKY_400};
  opacity: 0.35;
}}
polyline.sma2, polyline.sma4, polyline.sma8 {{
  visibility: hidden;
}}
polyline.trend {{
  stroke: {EMERALD_800};
  stroke-width: 0.4;
  stroke-dasharray: 1.25 0.75;
}}
text {{
  fill: {EMERALD_800};
  font-family: 'Source Sans Pro';
  font-size: 14%;
  font-weight: 400;
  text-anchor: middle;
  dominant-baseline: central;
  stroke: var(--bulma-body-background-color);
  stroke-width: 1%;
  paint-order: stroke;
  stroke-linejoin: round;
}}
"
        }
    }
}
