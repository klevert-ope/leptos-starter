use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <h1>"404"</h1>
        <p>"Page not found."</p>
    }
}
