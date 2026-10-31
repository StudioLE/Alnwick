#![cfg_attr(
    debug_assertions,
    expect(clippy::useless_format, reason = "dioxus#5635")
)]
pub use entry::start;

#[allow(dead_code)]
mod components;
mod contexts;
mod entry;
mod layout;
mod pages;
mod prelude;
mod route;
mod route_info;
mod utils;
