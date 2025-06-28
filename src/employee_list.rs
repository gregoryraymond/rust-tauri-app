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

#[derive(Clone)]
pub struct Item {
    pub title: String,
    pub name: String,
    pub image: String,
    pub role: String,
}

pub fn Card(item: Item) -> impl IntoView {
    view! {
        <li class="m-2 gap-8 divide-y divide-gray-200 rounded-lg bg-white shadow-xl/20">
        <div class="flex w-full items-center justify-between space-x-6 p-6">
            <div class="flex-1 truncate">
            <div class="flex items-center space-x-3">
                <h3 class="truncate text-sm font-medium text-gray-900">{item.name}</h3>
                <span class="inline-flex shrink-0 items-center rounded-full bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-green-600/20 ring-inset">{item.role}</span>
            </div>
            <p class="mt-1 truncate text-sm text-gray-500">{item.title}</p>
            </div>
            <img class="size-10 shrink-0 rounded-full bg-gray-300" src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=4&w=256&h=256&q=60" alt="" />
        </div>
        <div class="-mt-px flex divide-x divide-gray-200">
          <div class="flex w-0 flex-1">
            <a href="mailto:janecooper@example.com" class="relative -mr-px inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-bl-lg border border-transparent py-4 text-sm font-semibold text-gray-900">
              <svg class="size-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-slot="icon">
                <path d="M3 4a2 2 0 0 0-2 2v1.161l8.441 4.221a1.25 1.25 0 0 0 1.118 0L19 7.162V6a2 2 0 0 0-2-2H3Z" />
                <path d="m19 8.839-7.77 3.885a2.75 2.75 0 0 1-2.46 0L1 8.839V14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.839Z" />
              </svg>
              Email
            </a>
          </div>
          <div class="-ml-px flex w-0 flex-1">
            <a href="tel:+1-202-555-0170" class="relative inline-flex w-0 flex-1 items-center justify-center gap-x-3 rounded-br-lg border border-transparent py-4 text-sm font-semibold text-gray-900">
              <svg class="size-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-slot="icon">
                <path fill-rule="evenodd" d="M2 3.5A1.5 1.5 0 0 1 3.5 2h1.148a1.5 1.5 0 0 1 1.465 1.175l.716 3.223a1.5 1.5 0 0 1-1.052 1.767l-.933.267c-.41.117-.643.555-.48.95a11.542 11.542 0 0 0 6.254 6.254c.395.163.833-.07.95-.48l.267-.933a1.5 1.5 0 0 1 1.767-1.052l3.223.716A1.5 1.5 0 0 1 18 15.352V16.5a1.5 1.5 0 0 1-1.5 1.5H15c-1.149 0-2.263-.15-3.326-.43A13.022 13.022 0 0 1 2.43 8.326 13.019 13.019 0 0 1 2 5V3.5Z" clip-rule="evenodd" />
              </svg>
              Call
            </a>
          </div>
        </div>
        </li>
    }
}

#[component]
pub fn Cards() -> impl IntoView {
    let people: Vec<Item> = [
        Item {
            title: String::from("CEO"),
            name: String::from("Mr. Mann"),
            image: String::from(""),
            role: String::from("Big man"),
        },
        Item {
            title: String::from("CTO"),
            name: String::from("Fu Mann"),
            image: String::from(""),
            role: String::from("Mentor"),
        },
        Item {
            title: String::from("CMO"),
            name: String::from("Ma Mann"),
            image: String::from(""),
            role: String::from("Marketer"),
        },
        Item {
            title: String::from("CFMEO"),
            name: String::from("La Mann"),
            image: String::from(""),
            role: String::from("Friend"),
        },
        Item {
            title: String::from("CPO"),
            name: String::from("Ra Mann"),
            image: String::from(""),
            role: String::from("Tiger"),
        },
        Item {
            title: String::from("CTO"),
            name: String::from("Hu Mann"),
            image: String::from(""),
            role: String::from("Robot"),
        },
        Item {
            title: String::from("CTO"),
            name: String::from("Booo Mann"),
            image: String::from(""),
            role: String::from("Ghost"),
        },
    ]
    .to_vec();

    let cards = people.into_iter().map(|person| Card(person)).collect_view();
    view! {
        <main class="h-full justify-center bg-gray-50">
            <h1 class="text-2xl p-4 text-center text-shadow-xs">Employees</h1>
            <div class="flex">
                <ul role="list" class="p-2 w-full">
                    {cards}
                </ul>
                <button type="button" class="fixed bottom-12 right-8 rounded-full shadow-xl/20 drop-shadow-lg bg-gray-300 p-2 text-blue-950 active:bg-blue-400">
                    <svg class="size-14" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-slot="icon">
                        <path d="M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z" />
                    </svg>
                </button>
            </div>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (position, set_position) = signal(Position::default());
    let (zone, set_zone) = signal(String::default());

    let refresh = move |ev: SubmitEvent| {
        ev.prevent_default();
        // spawn_local(async move {
        //     let _ = connect_hk203().await;
        // });
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
        <Stylesheet id="leptos" href="/pkg/rust-tauri-app.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Cards/>
            </Routes>
        </Router>
    }
}
