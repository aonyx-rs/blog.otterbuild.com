use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn H3(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "mt-8 scroll-m-20 text-2xl font-semibold tracking-tight",
        class
    );

    view! { <h3 class=classes>{children()}</h3> }
}
