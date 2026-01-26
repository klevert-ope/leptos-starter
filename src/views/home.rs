use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    tracing::info!("Rendering HomePage");
    
    view! {
        <h1>"Home"</h1>
        <p>"Welcome to the Leptos starter."</p>
    }
}
