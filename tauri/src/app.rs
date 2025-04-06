use leptos::prelude::*;
use wasm_bindgen::prelude::*;

mod navbar;
use navbar::*;

mod feeds;
use feeds::*;

mod entries;
use entries::*;

mod content;
use content::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// THREE COLUMNS LAYOUT
#[component]
pub fn App() -> impl IntoView {
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
                    <EntriesHeader feed_name="Subscription" unread_count=1 />
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
