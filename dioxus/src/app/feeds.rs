use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;

// LEFT COLUMN
#[component]
fn Folder(folder_name: &'static str, feeds: &'static str) -> Element {
    let mut is_visible = use_signal(|| false);
    let mut folder_icon = use_signal(|| "folder");
    use_effect(move || {
        if is_visible() {
            folder_icon.set("folder_open");
        } else {
            folder_icon.set("folder");
        }
    });

    rsx! {
    div { onclick: move |_| is_visible.toggle(), class: "text-xs rounded-sm outline-none xl:text-sm collapse",
        input { aria_label: "folder name with unread count", r#type: "checkbox" }
        div { class: "collapse-title",
            Feed {
                feed_icon: folder_icon(),
                feed_name: folder_name,
                feed_unread_count: 3
            }
        }
        div { class: "ml-3 collapse-content",
            Feed { feed_name: "Yada Yada Boom", feed_unread_count: 1 }
            Feed { feed_name: "Dada Dada Boom", feed_unread_count: 1 }
            Feed { feed_name: "f0f0 f0f0 Boom", feed_unread_count: 1 }
        }
    }
    }
}

#[component]
fn Feed(
    #[props(default = "rss_feed")] feed_icon: &'static str,
    feed_name: &'static str,
    feed_unread_count: u32,
) -> Element {
    rsx! {
    div {
        aria_label: "feed name with unread count",
        aria_pressed: "false",
        class: "flex justify-between px-2 text-xs rounded-sm outline-none xl:text-sm focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content",
        role: "button",
        tabindex: "0",
        div { class: "flex py-1",
            i {
                aria_label: "feed icon, it can be a folder or rss symbol. the folder status can be open or closed. for the smart feed view, each category has its own icon associated.",
                class: "pr-2 mt-[2px]",
                MaterialIcon { name: feed_icon }
            }
            p { aria_label: "feed name", "{ feed_name }" }
        }
        div { aria_label: "unread count", class: "py-1",
            span { class: "badge badge-xs bg-base-300 xl:badge-sm", "{ feed_unread_count }" }
        }
    }
    }
}

#[component]
fn SmartFeeds() -> Element {
    rsx! {
        div {
            class: "px-2 pt-2 bg-base-200",
            aria_label: "smart feeds views: it includes dynamic views based on today's, unread, or starred (bookmarked), feeds.",
            p { class: "pb-1 text-xs font-semibold opacity-75", "Smart Feeds" }
        }
        div {
            class: "flex-col px-3",
            aria_label: "smart feeds categories with unread count. the count is dynamic based on their status.",
            Feed { feed_name: "Today", feed_icon: "feed", feed_unread_count: 1 }
            Feed { feed_name: "All Unread", feed_icon: "radio_button_checked", feed_unread_count: 1 }
            Feed { feed_name: "Starred", feed_icon: "star", feed_unread_count: 0 }
        }
    }
}

#[component]
fn FeedsAndFolders(account_name: &'static str) -> Element {
    rsx! {
        // Loop over accounts to show different collapsible accounts
        div {
            class: "sticky top-0 z-10 flex-col px-2 bg-base-200",
            aria_label: "specific account (e.g: iCloud) containing all feeds and subscriptions.",
            p { class: "text-xs font-semibold opacity-75", { account_name } }
        }

        div {
            class: "flex-col px-3 pt-1",
            aria_label: "all feeds belonging to a specific account, with unread count. the count is dynamic based on their unread status.",

            // Loop over account feeds to populate the list and make a distinction between feeds and folders
            Feed { feed_name: "Fake Random Communications", feed_unread_count: 1 }
            Folder { folder_name: "One Folder", feeds: "" }
            Folder{ folder_name: "Two Folder", feeds: "" }
         }
    }
}

#[component]
fn FeedsProgressBar() -> Element {
    rsx! {
    div {
        aria_label: "progress bar for all subscriptions refresh status. this is triggered by the user when they click the refresh button.",
        class: "flex-1 justify-around px-4 pt-4 border-t border-base-300",
        progress {
            class: "w-4/12 lg:w-6/12 xl:w-8/12 progress progress-info",
            max: "100",
            value: "42",
        }
        span { class: "w-1/12 text-xs font-semibold opacity-75 lg:pl-4 xl:pl-8", "42 of 100" }
    }}
}

#[component]
pub fn FeedsColumn() -> Element {
    rsx! {
    div {
        aria_label: "feeds column for all subscriptions and folders. this is where all feeds are displayed. the top smart feeds view shows today's feeds, unread feeds, and starred feeds.",
        class: "max-h-full select-none h-dvh bg-base-200",
        div { class: "justify-start", SmartFeeds {} }
        div {
            class: "overflow-y-auto overscroll-contain flex-col pt-2 h-17/24 lg:h-17/24 xl:h-19/24",
            FeedsAndFolders { account_name: "iCloud" }
        }
        div { class: "justify-end",  FeedsProgressBar {} }
    }}
}
