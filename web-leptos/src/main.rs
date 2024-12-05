use leptos::mount::mount_to_body;
use leptos::prelude::*;
// use phosphor_leptos::{Icon, IconWeight, CUBE, HEART, HORSE};
// https://github.com/SorenHolstHansen/phosphor-leptos
// https://phosphoricons.com/
use shared::Event;

mod core;
mod http;

#[component]
fn RootComponent() -> impl IntoView {
    let core = core::new();
    let (view, render) = signal(core.view());
    let (event, set_event) = signal(Event::StartWatch);

    Effect::new(move |_| {
        core::update(&core, event.get(), render);
    });

    view! {
        <section class="section has-text-centered">
            <p class="title">{"Crux Counter Example"}</p>
            <p class="is-size-5">{"Rust Core, Rust Shell (Leptos)"}</p>
        </section>
        <section class="container has-text-centered">
            <p class="is-size-5">{move || view.get().text}</p>
            <div class="buttons section is-centered">
                <button
                    class="button is-primary is-warning"
                    on:click=move |_| set_event.update(|value| *value = Event::Decrement)
                >
                    {"Decrement"}
                </button>
                <button
                    class="button is-primary is-danger"
                    on:click=move |_| set_event.update(|value| *value = Event::Increment)
                >
                    {"Increment"}
                </button>
                <ProgressBar progress=count/>
            </div>
        </section>
    }
}

/// Shows feeds refresh progress.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(RootComponent);
}
