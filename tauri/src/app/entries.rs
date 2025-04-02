use leptos::prelude::*;

// CENTER COLUMN
#[component]
fn Entries(feed_name: &'static str, feed_title: &'static str) -> impl IntoView {
    view! {
        <div
            class="overscroll-contain m-2 w-auto rounded-sm card card-xs prose-xs shadow-xs focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content"
            tabindex="0"
            role="button"
            aria-pressed="false"
            aria-label="single feed entry with title, published time, and first two lines of the content"
        >

            <div class="card-body">
                <div class="justify-between text-sm xl:text-base card-title">
                    <h2>{feed_title}</h2>
                    <span class="self-end mr-2 mb-2 w-2 h-2 status status-md status-info"></span>
                </div>
                <div>
                    <p class="text-xs xl:text-sm">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi malesuada posuere elementum. Praesent eget mollis est...
                    </p>
                </div>
            </div>
            <div class="justify-between mx-2 mr-3 mb-2 text-xs font-semibold opacity-75 card-actions">
                <p>{feed_name}</p>
                <p>10:37 AM</p>
            </div>
        </div>
    }
}

#[component]
pub fn EntriesColumn() -> impl IntoView {
    view! {
        <div class="max-h-full select-none h-dvh bg-base-100">
            <div class="overflow-y-auto overscroll-contain flex-col max-h-22/24">
                <Entries feed_name="Subscription" feed_title="Lorem Ipsum" />
            </div>
        </div>
    }
}
