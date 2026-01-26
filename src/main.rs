use leptos::{mount::mount_to_body, prelude::*};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod views;
mod layout;

fn main() {
    // Initialize tracing subscriber with environment filter and formatted output
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| String::from("leptos_starter=debug,info").into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Leptos application");

    mount_to_body(|| view! { <app::App /> })
}
