use leptos::prelude::*;
// use shared::Event;

mod navbar;
use navbar::*;

mod feeds;
use feeds::*;

mod entries;
use entries::*;

mod content;
use content::*;

// THREE COLUMNS LAYOUT
// #[component]
// fn RootComponent() -> impl IntoView {
//     view! {
//         <section class="box container has-text-centered m-5">
//             <p class="is-size-5">{move || view.get().count}</p>
//             <div class="buttons section is-centered">
//                 <button class="button is-primary is-danger"
//                     on:click=move |_| set_event.update(|value| *value = Event::Reset)
//                 >
//                     {"Reset"}
//                 </button>
//                 <button class="button is-primary is-success"
//                     on:click=move |_| set_event.update(|value| *value = Event::Increment)
//                 >
//                     {"Increment"}
//                 </button>
//                 <button class="button is-primary is-warning"
//                     on:click=move |_| set_event.update(|value| *value = Event::Decrement)
//                 >
//                     {"Decrement"}
//                 </button>
//             </div>
//         </section>
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    // let core = core::new();
    // let (view, render) = signal(core.view());
    // let (event, set_event) = signal(Event::Reset);

    // Effect::new(move |_| {
    // core::update(&core, event.get(), render);
    // });

    let feeds_classes = "basis-2/12 bg-base-200 py-1";
    let entries_classes = "basis-3/12 bg-base-100 py-1";
    let content_classes = "basis-7/12 bg-base-100 py-1";

    view! {
        <main class="overflow-hidden overscroll-none font-sans bg-base-100 text-base-content h-dvh">
            <div
                class="flex sticky top-0 z-10 flex-row border-b divide-x divide-base-300 border-base-300"
                aria-label="navigation bar with actionable commands and search bar."
            >
                <section class=feeds_classes>
                    <FeedsHeader />
                </section>
                <section class=entries_classes>
                    <EntriesHeader feed_name="Fake Random Communications" unread_count=1 />
                </section>
                <section class=content_classes>
                    <ContentHeader />
                </section>
            </div>
            <div
                class="flex flex-col h-full"
                aria-label="three columns containing: the feeds, the entries for the selected feed or view, the content (article/blog post) for a specific entry"
            >
                <div class="flex flex-row divide-x divide-base-300">
                    <section class=feeds_classes>
                        <FeedsColumn />
                    </section>
                    <section class=entries_classes>
                        <EntriesColumn />
                    </section>
                    <section class=content_classes>
                        <ContentColumn />
                    </section>
                </div>
            </div>
        </main>
    }
}
