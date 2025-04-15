use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn FeedsHeader() -> impl IntoView {
    view! {
        <div class="flex flex-row px-2 bg-base-200">
            <div class="flex-1 dropdown dropdown-bottom">
                <div
                    tabindex="0"
                    role="button"
                    class="text-sm xl:text-lg btn btn-ghost btn-square"
                    aria-label="add new..."
                >
                    <Icon icon=i::FaPlusSolid />
                </div>
                <ul
                    tabindex="0"
                    class="flex p-2 w-52 shadow-sm dropdown-content menu bg-base-100 rounded-box z-1"
                >
                    <li aria-label="add new account">
                        <p>
                            <Icon icon=i::FaUserPlusSolid />
                            New Account
                        </p>
                    </li>
                    <li aria-label="add new folder">
                        <p>
                            <Icon icon=i::FaFolderPlusSolid />
                            New Folder
                        </p>
                    </li>
                    <li aria-label="add new subscription">
                        <p>
                            <Icon icon=i::FaSquareRssSolid />
                            New Subscription
                        </p>
                    </li>
                </ul>
            </div>
            <div class="flex-none">
                <div class="mr-1 tooltip tooltip-bottom" aria-label="refresh all subscriptions">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">refresh</div>
                    </div>
                    <button
                        id="refresh-button"
                        aria-label="Refresh all subscriptions"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaArrowsRotateSolid />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EntriesHeader(feed_name: &'static str, unread_count: u32) -> impl IntoView {
    let count = format!("{} unread", unread_count);
    view! {
        <div class="flex flex-row px-2">
            <div class="flex-1 pt-1 pl-2">
                <p class="text-xs font-medium xl:text-sm" aria-label="feed name">
                    {feed_name}
                </p>
                <p class="text-xs opacity-75" aria-label="unread articles count">
                    {count}
                </p>
            </div>
            <div class="flex">
                <div class="tooltip tooltip-bottom" aria-label="mark all articles as read">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">mark all as read</div>
                    </div>
                    <button
                        id="mark-all-read-button"
                        aria-label="Mark all articles as read"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaGlassesSolid />
                    </button>
                </div>
                <div class="tooltip tooltip-bottom" aria-label="hide read articles">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">hide read articles</div>
                    </div>
                    <button
                        id="hide-read-articles-button"
                        aria-label="Hide all read articles"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaEyeSlashSolid />
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ContentHeader() -> impl IntoView {
    view! {
        <div class="flex flex-row px-2">
            <div class="flex-1">
                <div class="tooltip tooltip-bottom" aria-label="mark article as read">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">mark as read</div>
                    </div>
                    <button
                        id="mark-as-read-button"
                        aria-label="mark article as read"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaCircleDotRegular />
                    </button>
                </div>
                <div class="tooltip tooltip-bottom" aria-label="mark article as starred">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">mark as starred</div>
                    </div>
                    <button
                        id="mark-as-starred-button"
                        aria-label="mark article as starred"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaStarRegular />
                    </button>
                </div>
                <div class="tooltip tooltip-bottom" aria-label="next unread article">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">next unread</div>
                    </div>
                    <button
                        id="next-unread-button"
                        aria-label="next unread article"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaCircleRightRegular />
                    </button>
                </div>
                <div class="tooltip tooltip-bottom" aria-label="share article">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">share</div>
                    </div>
                    <button
                        id="share-article-button"
                        aria-label="share article"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaShareFromSquareRegular />
                    </button>
                </div>
                <div class="tooltip tooltip-bottom" aria-label="open article in browser">
                    <div class="tooltip-content">
                        <div class="text-xs xl:text-sm">open in browser</div>
                    </div>
                    <button
                        id="open-article-in-browser-button"
                        aria-label="open article in browser"
                        class="text-sm xl:text-lg btn btn-ghost btn-square"
                    >
                        <Icon icon=i::FaCompassRegular />
                    </button>
                </div>
            </div>
            <div class="flex pt-2 xl:pt-1" aria-label="search all feeds and articles">
                <label class="input input-xs xl:input-sm">
                    <Icon icon=i::FaMagnifyingGlassSolid />
                    <input type="search" class="grow" placeholder="Search" />
                </label>
            </div>
        </div>
    }
}
