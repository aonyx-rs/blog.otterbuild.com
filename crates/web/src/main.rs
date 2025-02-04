use leptos::prelude::*;

use crate::app::App;

mod app;
mod components;

fn main() {
    // Set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
