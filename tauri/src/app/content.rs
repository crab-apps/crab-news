use leptos::prelude::*;

// RIGHT COLUMN
#[component]
fn Content(
    feed_name: &'static str,
    feed_title: &'static str,
    feed_author: &'static str,
    date_time: &'static str,
    content: &'static str,
) -> impl IntoView {
    view! {
        <article
            class="m-auto w-10/12 max-w-4xl rounded-none md:rounded-sm xl:w-9/12 card prose-sm xl:prose-lg focus:bg-accent focus:text-accent-content active:bg-accent active:text-accent-content"
            tabindex="0"
            // role="button"
            aria-pressed="false"
            aria-label="actual article/blog post content with text, image, and video, including multimedia elements, such as audio and interactive elements"
        >
            <div class="card-body">
                <div class="justify-between -my-6 xl:-my-4 card-title">
                    <div class="flex-1 text-xs xl:text-sm">
                        <h3>{feed_name}</h3>
                        <h4 class="font-bold opacity-75">{feed_author}</h4>
                    </div>
                    <div class="mb-6 avatar">
                        <div class="w-20 xl:w-24">
                            <img
                                src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp"
                                alt="feed logo or author image"
                            />
                        </div>
                    </div>
                </div>

                <div class="divider"></div>

                <div class="text-sm xl:text-base">
                    <h1>{feed_title}</h1>
                    <h4 class="-mt-7 font-bold opacity-75 xl:-mt-9">{date_time}</h4>
                </div>

                <div class="w-full text-sm xl:text-base">
                    <p>{content}</p>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn ContentColumn() -> impl IntoView {
    view! {
        <div class="max-h-full h-dvh bg-base-100">
            <div class="overflow-y-auto overscroll-contain flex-col max-h-22/24">
                <Content
                    feed_name="Subscription"
                    feed_title="Lorem Ipsum"
                    feed_author="Jane Doe"
                    date_time="1 Apr 2025 at 10:37 AM"
                    content="
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent vulputate urna scelerisque, accumsan ex non, condimentum nunc. Donec ullamcorper velit cursus tellus fringilla tristique. Proin sollicitudin arcu vitae egestas consequat. Aenean elit ligula, vulputate at leo quis, vulputate eleifend ipsum. In convallis eros magna. Ut malesuada mauris ac sapien efficitur condimentum. Sed sodales, nibh id bibendum accumsan, lacus risus suscipit urna, eu vulputate dui neque a odio. Ut dignissim felis neque, eu vestibulum sem eleifend ut. Suspendisse tempor quam a lorem molestie aliquet et nec ligula. Nulla tincidunt sodales urna. Aliquam a feugiat purus, non fringilla est. Ut elit est, luctus ut egestas a, feugiat nec nibh. Nunc id viverra mauris. Duis eleifend facilisis sapien, at semper justo placerat nec. Sed elit sem, viverra vel hendrerit nec, accumsan varius ante.
                    
                    Nullam at eros eu dolor elementum facilisis. Nulla commodo, eros in egestas rhoncus, nibh ex congue felis, ac sollicitudin nulla justo ac sem. Sed posuere vitae orci eu auctor. Pellentesque sed congue ipsum. Ut aliquam purus enim, a bibendum est gravida sit amet. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi suscipit euismod urna, convallis facilisis nisl. Donec sed aliquam velit, eu volutpat elit. Praesent nulla ligula, pharetra id vehicula id, sodales et diam. Aenean rhoncus eros vitae justo luctus lacinia.
                    
                    Morbi sed egestas sapien, ac molestie turpis. Suspendisse venenatis sem quis lorem feugiat placerat. Curabitur sit amet ligula et tortor vulputate aliquet. Suspendisse commodo ornare vestibulum. Praesent aliquam lectus ut velit volutpat, eget tristique orci ultrices. In sodales tortor neque, nec rutrum tellus euismod at. Curabitur mi erat, auctor vel urna quis, posuere efficitur sem. Maecenas facilisis efficitur hendrerit.
                    "
                />
            </div>
        </div>
    }
}
