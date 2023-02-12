use super::{player::Mode, Model};
use once_cell::sync::Lazy;
use std::{sync::Mutex, time::Duration};
use wasm_bindgen::prelude::*;

static SIGNAL_RESET: Lazy<Mutex<(Mode, bool)>> = Lazy::new(|| Mutex::new((Mode::default(), false)));
static SIGNAL_LOAD: Lazy<Mutex<Option<SaveRequest>>> = Lazy::new(|| Mutex::new(None));

pub enum SaveRequest {
    RequestLoad { data: String },
    RequestSave,
    SaveData { data: String },
}

#[wasm_bindgen]
pub fn request_load(data: String) {
    if let Ok(mut mu_guard) = Lazy::force(&SIGNAL_LOAD).lock() {
        *mu_guard = Some(SaveRequest::RequestLoad { data })
    }
}

#[wasm_bindgen]
pub async fn request_store() -> String {
    if let Ok(mut mu_guard) = Lazy::force(&SIGNAL_LOAD).lock() {
        *mu_guard = Some(SaveRequest::RequestSave)
    }

    loop {
        async_std::task::sleep(Duration::from_millis(100)).await;
        if let Ok(mut mu_guard) = Lazy::get(&SIGNAL_LOAD).unwrap().lock() {
            let state = std::mem::take(&mut *mu_guard);
            if let Some(SaveRequest::SaveData { data }) = state {
                break data;
            } else {
                *mu_guard = state;
            }
        }
    }
}

#[wasm_bindgen]
pub fn signal_reset(mode: Mode) {
    if let Ok(mut mu_guard) = Lazy::force(&SIGNAL_RESET).lock() {
        *mu_guard = (mode, true)
    }
}

pub fn recieve_reset() -> Option<Mode> {
    Lazy::get(&SIGNAL_RESET)
        .and_then(|mu| mu.try_lock().ok())
        .and_then(|mut mu_guard| {
            mu_guard.1.then(|| {
                mu_guard.1 = false;
                mu_guard.0
            })
        })
}

pub fn respond_load_request(model: &mut Model) {
    if let Some(mut mu_guard) = Lazy::get(&SIGNAL_LOAD).and_then(|mu| mu.try_lock().ok()) {
        match std::mem::take(&mut *mu_guard) {
            Some(SaveRequest::RequestLoad { data }) => {
                model.board = ron::from_str(&data).unwrap();
            }
            Some(SaveRequest::RequestSave) => {
                *mu_guard = Some(SaveRequest::SaveData {
                    data: ron::to_string(&model.board).unwrap(),
                })
            }
            None => (),
            x => {
                *mu_guard = x;
            }
        }
    }
}
