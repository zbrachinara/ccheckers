#![cfg_attr(target_arch = "wasm32", allow(unused))]

use common::{events, model, update};

mod common;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    nannou::app(model).event(events).update(update).run()
}
