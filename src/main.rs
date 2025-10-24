mod app;
mod chart;
mod entries;
mod extensions;
mod import;
mod nav;
mod prelude;
mod settings;
mod state;
mod statistics;
mod table;

use crate::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
