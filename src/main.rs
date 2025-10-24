mod app;
mod nav;
mod prelude;
mod settings;
mod state;
mod table;
mod pages;
mod layout;
mod core;
mod contexts;

use crate::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
