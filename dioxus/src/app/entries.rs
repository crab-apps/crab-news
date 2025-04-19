use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

// CENTER COLUMN
#[component]
fn Entry(
    entry_title: &'static str,
    entry_author: &'static str,
    entry_time: &'static str,
    entry_summary: &'static str,
) -> Element {
    let authors = format!("by {}", entry_author);

    rsx! {
    div {
        aria_label: "single feed entry with title, published time, and first two lines of the content",
        aria_pressed: "false",
        class: "overscroll-contain m-2 w-auto rounded-sm card card-xs prose-xs shadow-xs focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content",
        role: "button",
        tabindex: "0",
        div { class: "card-body",
            div { class: "justify-between text-xs xl:text-sm card-title",
                h2 { class: "flex-1", "{ entry_title }" }
                span { class: "self-end mr-2 mb-3 w-2 h-2",  MaterialIcon { name: "star" } }
                span { class: "self-end mr-2 mb-2 w-2 h-2 status status-md status-info" }
            }
            div {
                p { class: "text-xs xl:text-sm", "{ entry_summary }" }
            }
        }
        div { class: "justify-between mx-2 mr-3 mb-2 text-xs font-semibold opacity-75 card-actions",
            p { "{ authors }" }
            p { "{ entry_time }" }
        }
    }
    }
}

#[component]
pub fn EntriesColumn() -> Element {
    rsx! {
        div { class: "max-h-full select-none h-dvh bg-base-100",
            div {
                aria_label: "all entries for the selected feed or smart view category",
                class: "overflow-y-auto overscroll-contain flex-col max-h-22/24",

                Entry {
                    entry_title: "Lorem ipsum dolor sit amet, consectetur adipiscing ...",
                    entry_author: "Jane Doe, John Smith, Jane Smith, David Smith...",
                    entry_time: "10:37 AM",
                    entry_summary: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl eget aliquam aliquet, nunc nisl aliquet nisl, ..."
                }

                Entry {
                    entry_title: "MMXXV",
                    entry_author: "Gentle Wash Records",
                    entry_time: "9 Jan 2025",
                    entry_summary: "All tracks originally composed for Raster Media Europe 2025 open call. Credits Field Notes Svatur. Dada umpa blisset, ..."
                }
            }
        }
    }
}
