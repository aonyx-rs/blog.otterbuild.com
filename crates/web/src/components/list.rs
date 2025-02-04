use leptos::either::either;
use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "my-6 ml-6 [&>li]:mt-2")]
struct ListStyle {
    list_type: ListType,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, TwVariant)]
pub enum ListType {
    #[tw(default, class = "list-decimal")]
    Ordered,
    #[tw(class = "list-disc")]
    Unordered,
}

#[component]
pub fn List(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] list_type: ListType,
    // TODO: Can we limit this to only accept ListItems?
    children: Children,
) -> impl IntoView {
    let final_class = ListStyle { list_type }.with_class(class);

    either!(list_type,
        ListType::Ordered => view! { <ol class=final_class>{children()}</ol> },
        _ => view! { <ul class=final_class>{children()}</ul> },
    )
}
