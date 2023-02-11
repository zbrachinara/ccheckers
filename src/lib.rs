#![cfg(target_arch = "wasm32")]

use async_std::task::block_on;
use nannou::{prelude::*, wgpu::Backends};
use wasm_bindgen::prelude::wasm_bindgen;

mod common;

#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        app::Builder::new_async(|app| Box::new(common::model(app)))
            .backends(Backends::PRIMARY | Backends::GL)
            .event(common::events)
            .run_async()
            .await
    });
}
