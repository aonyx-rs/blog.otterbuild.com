use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn H2(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "mt-10 scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        class
    );

    view! { <h2 class=classes>{children()}</h2> }
}
