use dioxus::prelude::*;
use shared::{Event, ViewModel};

mod core;
use core::CoreService;
mod http;

mod navbar;
use navbar::*;

mod feeds;
use feeds::*;

mod entries;
use entries::*;

mod content;
use content::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let view = use_signal(ViewModel::default);

    let core = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view);
        async move { svc.run(&mut rx).await }
    });

    let feeds_classes = "basis-2/12 bg-base-200 py-1";
    let entries_classes = "basis-3/12 bg-base-100 py-1";
    let content_classes = "basis-7/12 bg-base-100 py-1";

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main { class: "overflow-hidden overscroll-none font-sans bg-base-100 text-base-content h-dvh",
            div {
                aria_label: "navigation bar with actionable commands and search bar.",
                class: "flex sticky top-0 z-10 flex-row border-b divide-x divide-base-300 border-base-300",
                section { class: feeds_classes,
                    FeedsHeader {}
                }
                section { class: entries_classes,
                    EntriesHeader {
                        feed_name: "Fake Random Communications",
                        unread_count: 1
                    }
                }
                section { class: content_classes ,
                    ContentHeader {}
                }
            }
            div {
                aria_label: "three columns containing: the feeds, the entries for the selected feed or view, the content (article/blog post) for a specific entry",
                class: "flex flex-col h-full",
                div { class: "flex flex-row divide-x divide-base-300",
                    section { class: feeds_classes,
                        FeedsColumn {}
                    }
                    section { class: entries_classes,
                        EntriesColumn {}
                    }
                    section { class: content_classes,
                        ContentColumn {}
                    }
                }
            }
        }
    }
}
