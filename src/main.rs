mod contexts;
mod core;
mod layout;
mod pages;
mod prelude;

use crate::layout::app::App;
use dioxus::prelude::launch;

fn main() {
    launch(App);
}
