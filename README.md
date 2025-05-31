# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

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
