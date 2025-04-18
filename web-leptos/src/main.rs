use leptos::mount::mount_to_body;
use leptos::view;
use shared::Event;

mod app;
use app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
