mod prelude;
mod state;
mod pages;
mod layout;
mod core;
mod contexts;

use crate::layout::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
