use leptos::{mount::mount_to_body, prelude::*};

mod app;
mod pages;
mod layout;

fn main() {
    mount_to_body(|| view! { <app::App /> })
}
