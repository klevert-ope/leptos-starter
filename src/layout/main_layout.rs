use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="app-container">
            <header>
                <nav>
                    <A href="/">"Home"</A>
                    " | "
                    <A href="/about">"About"</A>
                </nav>
            </header>

            <main>
                {children()}
            </main>
        </div>
    }
}
