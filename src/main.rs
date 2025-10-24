mod app;
mod prelude;
mod state;
mod pages;
mod layout;
mod core;
mod contexts;

use crate::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
