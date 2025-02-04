use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn H1(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        class
    );

    view! { <h1 class=classes>{children()}</h1> }
}
