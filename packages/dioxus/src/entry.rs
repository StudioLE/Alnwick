use crate::layout::App;
use crate::prelude::*;
use dioxus::launch;

pub fn start() {
    #[cfg(not(target_arch = "wasm32"))]
    init_server();
    #[cfg(target_arch = "wasm32")]
    init_wasm();
    launch(App);
}
