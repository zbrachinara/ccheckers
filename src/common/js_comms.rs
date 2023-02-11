use super::player::Mode;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static SIGNAL: Lazy<Mutex<(Mode, bool)>> = Lazy::new(|| Mutex::new((Mode::default(), false)));

#[wasm_bindgen]
pub fn signal_reset(mode: Mode) {
    if let Ok(mut mu_guard) = Lazy::force(&SIGNAL).lock() {
        *mu_guard = (mode, true)
    }
}

pub fn recieve_reset() -> Option<Mode> {
    Lazy::get(&SIGNAL)
        .and_then(|mu| mu.try_lock().ok())
        .and_then(|mut mu_guard| {
            mu_guard.1.then(|| {
                mu_guard.1 = false;
                mu_guard.0
            })
        })
}
