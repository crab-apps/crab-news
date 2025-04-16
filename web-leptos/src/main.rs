use leptos::prelude::*;

// mod core;
// mod http;

mod app;
use app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
