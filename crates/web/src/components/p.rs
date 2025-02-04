use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn P(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("leading-7 [&:not(:first-child)]:mt-6", class);

    view! { <p class=classes>{children()}</p> }
}
