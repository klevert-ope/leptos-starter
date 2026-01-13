use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::StaticSegment;

use crate::layout::main_layout::MainLayout;
use crate::pages::{HomePage, AboutPage, NotFoundPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <MainLayout>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=() view=HomePage />
                    <Route path=(StaticSegment("about"),) view=AboutPage />
                </Routes>
            </MainLayout>
        </Router>
    }
}