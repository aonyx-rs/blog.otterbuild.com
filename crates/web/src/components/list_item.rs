use leptos::prelude::*;

#[component]
pub fn ListItem(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <li class=class>{children()}</li> }
}
