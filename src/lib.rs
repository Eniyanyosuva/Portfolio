use wasm_bindgen::prelude::*;

mod app;
mod footer;
mod header;
mod tabs;

#[wasm_bindgen]
pub fn hydrate() {}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
