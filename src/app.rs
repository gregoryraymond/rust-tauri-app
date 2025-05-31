use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use tauri_ipc_macros::invoke_bindings;
use crate::position::Position;
use wasm_bindgen::prelude::*;

#[allow(async_fn_in_trait)]
#[invoke_bindings]
#[allow(dead_code)]
pub trait Commands {
    async fn get_current_position() -> Result<Position, String>;
    async fn connect_hk203() -> Result<(), String>;
    async fn get_current_zone(p: Position) -> Result<String, String>;
}

#[component]
pub fn App() -> impl IntoView {
    let (position, set_position) = signal(Position::default());
    let (zone, set_zone) = signal(String::default());

    let refresh = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = connect_hk203().await;
        });
        spawn_local(async move {
            if let Ok(post) = get_current_position().await {
                set_position.set(post.clone());
                if let Ok(zone) = get_current_zone(post).await {
                    set_zone.set(zone);
                }
            }
        });
    };

    view! {
        <main class="container">
            <form class=(["row"], true) on:submit=refresh>
                <button type="submit">"Refresh"</button>
            </form>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div>
                        <ul>
                        { move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { <li> { e.to_string() } </li> })
                            .collect::<Vec<_>>()
                        }
                        </ul>
                    </div>
                }
            >
            <p>{ move || format!("Zone: {}", zone.get()) } </p>
            <p>{ move || format!("Coords: {},{}", position.get().coords.latitude, position.get().coords.longitude) } </p>
            </ErrorBoundary>
        </main>
    }
}
