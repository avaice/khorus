mod audio;
mod default_pack;
mod keyboard;
mod keys;
mod pack;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::audio::AudioEngine;
use crate::pack::PackInfo;

struct AppState {
    enabled: Arc<AtomicBool>,
    pack: PackInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    enabled: bool,
    permission_granted: bool,
    pack: PackInfo,
}

#[tauri::command]
fn get_status(state: State<AppState>) -> Status {
    Status {
        enabled: state.enabled.load(Ordering::SeqCst),
        permission_granted: keyboard::has_permission(),
        pack: state.pack.clone(),
    }
}

#[tauri::command]
fn set_enabled(state: State<AppState>, enabled: bool) {
    state.enabled.store(enabled, Ordering::SeqCst);
}

#[tauri::command]
fn request_permission() -> bool {
    keyboard::request_permission()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let enabled = Arc::new(AtomicBool::new(true));
    let audio = AudioEngine::spawn();
    let default_pack = default_pack::load().expect("デフォルトのパックが不正です");
    let pack = default_pack.info.clone();
    audio.set_pack(default_pack);

    let listener_enabled = Arc::clone(&enabled);
    keyboard::spawn(move |key| {
        if listener_enabled.load(Ordering::SeqCst) {
            audio.play(key);
        }
    });

    tauri::Builder::default()
        .manage(AppState { enabled, pack })
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_enabled,
            request_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
