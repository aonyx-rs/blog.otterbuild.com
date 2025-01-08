use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // Set the document title
        <Title text="Welcome to Otter 🦦" />

        // Inject metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <p>"Hello from Otter! 🦦"</p>
    }
}
