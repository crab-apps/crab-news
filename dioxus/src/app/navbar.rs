use super::core::CoreService;
use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;
use shared::{Event, ViewModel};

#[component]
pub fn FeedsHeader() -> Element {
    let view = use_signal(ViewModel::default);
    let core = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view);
        async move { svc.run(&mut rx).await }
    });

    rsx! {
        div { class: "flex flex-row px-2 bg-base-200",
            div { class: "flex-1 dropdown dropdown-bottom",
                div {
                    aria_label: "add new...",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    role: "button",
                    tabindex: "0",
                    MaterialIcon { name: "add" }
                }
                ul {
                    class: "flex w-52 shadow-sm dropdown-content menu bg-base-100 rounded-box z-1",
                    tabindex: "0",
                    li { aria_label: "add new account",
                        p {
                            MaterialIcon { name: "person_add" }
                            " New Account "
                        }
                    }
                    li { aria_label: "add new folder",
                    p {
                        MaterialIcon { name: "create_new_folder" }
                        " New Folder "
                    }
                    }
                    li { aria_label: "add new subscription",
                    p {
                        MaterialIcon { name: "add_circle_outline" }
                        " New Subscription "
                    }
                    }
                }
            }
            div { class: "flex-none",
                div {
                aria_label: "refresh all subscriptions",
                class: "mr-1 tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "refresh" }
                }
                button {
                    aria_label: "Refresh all subscriptions",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "refresh-button",
                    MaterialIcon { name: "refresh" }
                }
                }
            }
        }
    }
}

#[component]
pub fn EntriesHeader(feed_name: &'static str, unread_count: u32) -> Element {
    let count = format!("{} unread", unread_count);

    rsx! {
    div { class: "flex flex-row px-2",
        div { class: "flex-1 pt-1 pl-2",
            p {
                aria_label: "feed name",
                class: "text-xs font-medium xl:text-sm",
                " { feed_name } "
            }
            p {
                aria_label: "unread articles count",
                class: "text-xs opacity-75",
                " { count } "
            }
        }
        div { class: "flex",
            div {
                aria_label: "mark all articles as read",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "mark all as read" }
                }
                button {
                    aria_label: "Mark all articles as read",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "mark-all-read-button",
                    MaterialIcon { name: "done_all" }
                }
            }
            div {
                aria_label: "hide read articles",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "hide read articles" }
                }
                button {
                    aria_label: "Hide all read articles",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "hide-read-articles-button",
                    MaterialIcon { name: "visibility_off" }
                }
            }
        }
    }
    }
}

#[component]
pub fn ContentHeader() -> Element {
    rsx! {
    div { class: "flex flex-row px-2",
        div { class: "flex-1",
            div {
                aria_label: "mark article as read",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "mark as read" }
                }
                button {
                    aria_label: "mark article as read",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "mark-as-read-button",
                    MaterialIcon { name: "check" }
                }
            }
            div {
                aria_label: "mark article as starred",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "mark as starred" }
                }
                button {
                    aria_label: "mark article as starred",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "mark-as-starred-button",
                    MaterialIcon { name: "star_border" }
                }
            }
            div {
                aria_label: "next unread article",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "next unread" }
                }
                button {
                    aria_label: "next unread article",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "next-unread-button",
                    MaterialIcon { name: "expand_circle_down" }
                }
            }
            div { aria_label: "share article", class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "share" }
                }
                button {
                    aria_label: "share article",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "share-article-button",
                    MaterialIcon { name: "share" }
                }
            }
            div {
                aria_label: "open article in browser",
                class: "tooltip tooltip-bottom",
                div { class: "tooltip-content",
                    div { class: "text-xs xl:text-sm", "open in browser" }
                }
                button {
                    aria_label: "open article in browser",
                    class: "text-sm xl:text-lg btn btn-ghost btn-square",
                    id: "open-article-in-browser-button",
                    MaterialIcon { name: "open_in_new" }
                }
            }
        }
        div {
            aria_label: "search all feeds and articles",
            class: "flex pt-2 xl:pt-1",
            label { class: "input input-xs xl:input-sm text-sm xl:text-sm",
                MaterialIcon { name: "search" }
                input { class: "grow", placeholder: "Search", r#type: "search" }
            }
        }
    }
    }
}
