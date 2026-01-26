use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    tracing::info!("Rendering AboutPage");
    
    view! {
        <h1>"About"</h1>
        <p>"This starter supports multiple views and layouts."</p>
    }
}
