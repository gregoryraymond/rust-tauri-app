# Tauri + Leptos + Tailwindcss
 
Tauri + Leptos + Tailwindcss compiling for android app.

The attempt of this project is to create an app that automates certain aspects of my commute.

Namely, is able to:

* Connect to bluetooth when the journey begins.
* Autoplay my podcast once connected to bluetooth.
* Autonavigate based on a tiny amount of input.
* Turn on wifi when the journey ends and disable bluetooth.
* Lock my doggy door remotely as I arrive home.

Some of this is necessary because of some of the restrictions I have placed on my mobile device.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Prereqs

Need to install pre-reqs of tauri: https://v2.tauri.app/start/prerequisites/#configure-for-mobile-targets

Then:
`cargo install trunk`
`cargo install tauri-cli --version "^2.0.0" --locked`
`cargo install wasm-bindgen-cli`

`rustup default nightly`
`rustup target add wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`

`cargo tauri add fs`
`cargo tauri add http`
