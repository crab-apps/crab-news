use dioxus::prelude::*;

// RIGHT COLUMN
#[component]
fn Content(
    feed_name: &'static str,
    feed_logo: &'static str,
    content_title: &'static str,
    content_time: &'static str,
    content_author: &'static str,
    content_content: &'static str,
) -> Element {
    let authors = format!("by {}", content_author);
    let on_subscribe = format!("on {}", feed_name);

    rsx! {
        article {
            aria_label: "actual article/blog post content with text, image, and video, including multimedia elements, such as audio and interactive elements",
            aria_pressed: "false",
            class: "m-auto w-10/12 max-w-4xl rounded-none md:rounded-sm xl:w-9/12 card prose-sm xl:prose-lg",
            role: "article",
            tabindex: "0",
            div { class: "card-body",
                div { class: "card-title",
                    div { class: "flex-1 text-xs xl:text-sm",
                        h1 { class: "max-w-11/12", "{content_title}" }
                        p { class: "-mt-4 font-bold opacity-75 xl:-mt-7", "{content_time}" }
                    }
                    div { class: "mb-12 avatar",
                        div { class: "w-16 rounded xl:w-20",
                            img {
                                alt: "subscription logo or avatar",
                                src: "{feed_logo}",
                            }
                        }
                    }
                }
                div { class: "-mt-8 divider" }
                div { class: "justify-between -mt-8 text-xs font-bold opacity-75 xl:text-sm card-actions",
                    p { class: "max-w-7/12", "{authors}" }
                    p { class: "text-right max-w-4/12", "{on_subscribe}" }
                }
                div { class: "w-full text-sm xl:text-base",
                    p { "{content_content}" }
                }
            }
        }
    }
}

#[component]
pub fn ContentColumn() -> Element {
    rsx! {
        div { class: "max-h-full h-dvh bg-base-100",
            div {
                aria_label: "this is where the content of the selected entry is displayed",
                class: "overflow-y-auto overscroll-contain flex-col max-h-22/24",
                Content {
                    feed_name: "Fake Random Communications",
                    feed_logo: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp",
                    content_title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
                    content_time: "1 Apr 2025 at 10:37 AM",
                    content_author: "Jane Doe, John Smith, Jane Smith, David Smith, John Doe, William Smith, Emily Johnson, Sarah Johnson, Michael Smith",
                    content_content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent vulputate urna scelerisque, accumsan ex non, condimentum nunc. Donec ullamcorper velit cursus tellus fringilla tristique. Proin sollicitudin arcu vitae egestas consequat. Aenean elit ligula, vulputate at leo quis, vulputate eleifend ipsum. In convallis eros magna. Ut malesuada mauris ac sapien efficitur condimentum. Sed sodales, nibh id bibendum accumsan, lacus risus suscipit urna, eu vulputate dui neque a odio. Ut dignissim felis neque, eu vestibulum sem eleifend ut. Suspendisse tempor quam a lorem molestie aliquet et nec ligula. Nulla tincidunt sodales urna. Aliquam a feugiat purus, non fringilla est. Ut elit est, luctus ut egestas a, feugiat nec nibh. Nunc id viverra mauris. Duis eleifend facilisis sapien, at semper justo placerat nec. Sed elit sem, viverra vel hendrerit nec, accumsan varius ante.                    Nullam at eros eu dolor elementum facilisis. Nulla commodo, eros in egestas rhoncus, nibh ex congue felis, ac sollicitudin nulla justo ac sem. Sed posuere vitae orci eu auctor. Pellentesque sed congue ipsum. Ut aliquam purus enim, a bibendum est gravida sit amet. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi suscipit euismod urna, convallis facilisis nisl. Donec sed aliquam velit, eu volutpat elit. Praesent nulla ligula, pharetra id vehicula id, sodales et diam. Aenean rhoncus eros vitae justo luctus lacinia.             Morbi sed egestas sapien, ac molestie turpis. Suspendisse venenatis sem quis lorem feugiat placerat. Curabitur sit amet ligula et tortor vulputate aliquet. Suspendisse commodo ornare vestibulum. Praesent aliquam lectus ut velit volutpat, eget tristique orci ultrices. In sodales tortor neque, nec rutrum tellus euismod at. Curabitur mi erat, auctor vel urna quis, posuere efficitur sem. Maecenas facilisis efficitur hendrerit.",
                }
            }
        }
    }
}
