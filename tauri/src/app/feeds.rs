use icondata as i;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_element_visibility;

// LEFT COLUMN
#[component]
fn Folder(folder_name: &'static str, feeds: &'static str) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let is_visible = use_element_visibility(el);

    view! {
        <div class="text-xs rounded-sm outline-none xl:text-sm collapse">
            <input type="checkbox" aria-label="folder name with unread count" />
            <div class="collapse-title">
                <Show
                    when=move || { is_visible == true.into() }
                    fallback=move || {
                        view! {
                            <Feed
                                feed_icon=i::FaFolderOpenRegular
                                feed_name=folder_name
                                feed_unread_count=3
                            />
                        }
                    }
                >
                    <Feed
                        feed_icon=i::FaFolderClosedRegular
                        feed_name=folder_name
                        feed_unread_count=3
                    />
                </Show>
            </div>
            <div node_ref=el class="collapse-content">
                <Feed feed_name="Yada Yada Boom" feed_unread_count=1 />
                <Feed feed_name="Dada Dada Boom" feed_unread_count=1 />
                <Feed feed_name="f0f0 f0f0 Boom" feed_unread_count=1 />
            </div>
        </div>
    }
}

#[component]
fn Feed(
    #[prop(default = i::FaSquareRssSolid)] feed_icon: i::Icon,
    feed_name: &'static str,
    feed_unread_count: u32,
) -> impl IntoView {
    view! {
        <div
            class="flex justify-between px-2 text-xs rounded-sm outline-none xl:text-sm focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content"
            tabindex="0"
            role="button"
            aria-pressed="false"
            aria-label="feed name with unread count"
        >
            <div class="flex py-1">
                <i
                    class="pr-2 mt-[2px]"
                    aria-label="feed icon, it can be a folder or rss symbol. the folder status can be open or closed. for the smart feed view, each category has its own icon associated."
                >
                    <Icon icon=feed_icon />
                </i>
                <p aria-label="feed name">{feed_name}</p>
            </div>
            <div class="py-1" aria-label="unread count">
                <span class="badge badge-xs bg-base-300 xl:badge-sm">{feed_unread_count}</span>
            </div>
        </div>
    }
}

#[component]
fn SmartFeeds() -> impl IntoView {
    view! {
        <div
            class="px-2 pt-2 bg-base-200"
            aria-label="smart feeds views: it includes dynamic views based on today's, unread, or starred (bookmarked), feeds."
        >
            <p class="pb-1 text-xs font-semibold opacity-75">Smart Feeds</p>
        </div>
        <div
            class="flex-col px-3"
            aria-label="smart feeds categories with unread count. the count is dynamic based on their status."
        >
            <Feed feed_name="Today" feed_icon=i::FaNewspaperSolid feed_unread_count=1 />
            <Feed feed_name="All Unread" feed_icon=i::FaCircleDotSolid feed_unread_count=1 />
            <Feed feed_name="Starred" feed_icon=i::FaStarSolid feed_unread_count=0 />
        </div>
    }
}

#[component]
fn FeedsAndFolders() -> impl IntoView {
    view! {
        // Loop over accounts to show different collapsible accounts
        <div
            class="sticky top-0 z-10 flex-col px-2 bg-base-200"
            aria-label="specific account (e.g: iCloud) containing all feeds and subscriptions."
        >
            // Account name
            <p class="text-xs font-semibold opacity-75">iCloud</p>
        </div>
        <div
            class="flex-col px-3 pt-1"
            aria-label="all feeds belonging to a specific account, with unread count. the count is dynamic based on their unread status."
        >
            // Loop over account feeds to populate the list and make a distinction between feeds and folders
            <Feed feed_name="Fake Random Communications" feed_unread_count=1 />
            <Folder folder_name="One Folder" feeds="" />
            <Folder folder_name="Two Folder" feeds="" />
        </div>
    }
}

#[component]
fn FeedsProgressBar() -> impl IntoView {
    view! {
        <div
            class="flex-1 justify-around px-4 pt-10 xl:pt-6"
            aria-label="progress bar for all subscriptions refresh status. this is triggered by the user when they click the refresh button."
        >
            <progress
                class="w-4/12 lg:w-6/12 xl:w-8/12 progress progress-info"
                value="42"
                max="100"
            ></progress>
            <span class="w-1/12 text-xs font-semibold opacity-75 lg:pl-4 xl:pl-8">42 of 100</span>
        </div>
    }
}

#[component]
pub fn FeedsColumn() -> impl IntoView {
    view! {
        <div
            class="max-h-full select-none h-dvh bg-base-200"
            aria-label="feeds column for all subscriptions and folders. this is where all feeds are displayed. the top smart feeds view shows today's feeds, unread feeds, and starred feeds."
        >
            <div class="justify-start">
                <SmartFeeds />
            </div>
            <div class="overflow-y-auto overscroll-contain flex-col pt-2 h-17/24 lg:h-17/24 xl:h-19/24">
                <FeedsAndFolders />
            </div>
            <div class="justify-end">
                <FeedsProgressBar />
            </div>
        </div>
    }
}
