use crate::employee_list::Cards;
use crate::position::Position;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use tauri_ipc_macros::invoke_bindings;
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
pub fn Header() -> impl IntoView {
    
    let (position, set_position) = signal(Position::default());
    let (zone, set_zone) = signal(String::default());

    let combined = move || {
        let coords = position.get().coords;
        print!("coords: {},{}; zone: {}", coords.latitude, coords.longitude, zone.get())
    };

    let refresh = move || {
        spawn_local(async move {
            let _ = connect_hk203().await;
        });
        spawn_local(async move {
            let array = js_sys::Array::of2(
                &JsValue::from_str("location"),
                &JsValue::from_str("coarseLocation"),
            );
            let _res = invoke(&"plugin:geolocation|request_permissions", array.into()).await;
            let pos = get_current_position().await;
            leptos::logging::log!("Position {:#?}", pos);
            if let Ok(post) = pos {
                set_position.set(post.clone());
                if let Ok(zone) = get_current_zone(post).await {
                    set_zone.set(zone);
                }
            }
        });
    };

    view! {
        <div class="flex">
            <h2>{combined}</h2>
            <button 
            type="button" 
            on:click=move |_| refresh()
            class="fixed bottom-12 right-8 rounded-full shadow-xl/20 drop-shadow-lg bg-gray-300 p-2 text-blue-950 active:bg-blue-400">
            </button>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/rust-tauri-app.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                // <Route path=StaticSegment("") view=Header/>
                <Route path=StaticSegment("") view=Cards/>
            </Routes>
        </Router>
    }
}
