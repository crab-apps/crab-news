use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

// LEFT COLUMN
#[component]
fn Feed(icon: i::Icon, feed: &'static str, unread_count: u32) -> impl IntoView {
    view! {
        <div
            class="flex justify-between px-2 text-xs rounded-sm outline-none xl:text-sm focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content"
            tabindex="0"
            role="button"
            aria-pressed="false"
            aria-label="feed name with unread count"
        >
            <div class="flex py-1">
                <i class="pr-2 mt-[2px]">
                    <Icon icon=icon />
                </i>
                <p>{feed}</p>
            </div>

            <div class="py-1" aria-label="unread count">
                <span class="badge badge-xs bg-base-300 xl:badge-sm">{unread_count}</span>
            </div>
        </div>
    }
}

#[component]
fn SmartFeeds() -> impl IntoView {
    view! {
        <div class="px-2 pt-2 bg-base-200">
            <p class="pb-1 text-xs font-semibold opacity-75">Smart Feeds</p>
        </div>
        <div class="flex-col px-3">
            <Feed feed="Today" icon=i::FaNewspaperSolid unread_count=1 />
            <Feed feed="All Unread" icon=i::FaCircleDotSolid unread_count=1 />
            <Feed feed="Starred" icon=i::FaStarSolid unread_count=0 />
        </div>
    }
}

#[component]
fn FeedsAndFolders() -> impl IntoView {
    view! {
        // Loop over accounts to show different collapsible accounts
        <div class="sticky top-0 z-10 flex-col px-2 bg-base-200">
            // Account name
            <p class="text-xs font-semibold opacity-75">iCloud</p>
        </div>
        <div class="flex-col px-3 pt-1">
            // Loop over account feeds to populate the list
            <Feed feed="Subscription" icon=i::FaSquareRssSolid unread_count=1 />
            <Feed feed="Close Folder" icon=i::FaFolderClosedRegular unread_count=0 />
            <Feed feed="Open Folder" icon=i::FaFolderOpenRegular unread_count=0 />
        </div>
    }
}

#[component]
fn FeedsProgressBar() -> impl IntoView {
    view! {
        <div
            class="flex-1 justify-around px-4 pt-10 xl:pt-6"
            aria-label="progress bar for all subscriptions refresh status"
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
        <div class="max-h-full select-none h-dvh bg-base-200">
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
