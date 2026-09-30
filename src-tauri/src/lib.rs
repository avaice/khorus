mod archive;
mod audio;
mod builtin;
mod keyboard;
mod keys;
mod library;
mod pack;
mod settings;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use serde::Serialize;
use tauri::menu::{Menu, SubmenuBuilder};
use tauri::{Manager, State};

use crate::audio::AudioEngine;
use crate::library::Library;
use crate::settings::{Settings, SettingsStore};

struct AppState {
    enabled: Arc<AtomicBool>,
    audio: AudioEngine,
    library: Library,
    settings: SettingsStore,
    selected: Mutex<String>,
}

impl AppState {
    fn selected_id(&self) -> String {
        self.selected
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    fn activate(&self, id: &str) -> Result<(), String> {
        let pack = self.library.load(id).map_err(|error| error.to_string())?;
        self.audio.set_pack(pack);
        *self.selected.lock().unwrap_or_else(PoisonError::into_inner) = id.to_string();
        self.settings
            .save(&Settings {
                selected_pack: Some(id.to_string()),
            })
            .map_err(|error| error.to_string())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    enabled: bool,
    permission_granted: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PackSummary {
    id: String,
    title: String,
    description: String,
    builtin: bool,
    selected: bool,
}

#[tauri::command]
fn get_status(state: State<AppState>) -> Status {
    Status {
        enabled: state.enabled.load(Ordering::SeqCst),
        permission_granted: keyboard::has_permission(),
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

#[tauri::command]
fn list_packs(state: State<AppState>) -> Vec<PackSummary> {
    let selected = state.selected_id();
    state
        .library
        .entries()
        .into_iter()
        .map(|entry| PackSummary {
            selected: entry.id == selected,
            id: entry.id,
            title: entry.info.title,
            description: entry.info.description,
            builtin: entry.builtin,
        })
        .collect()
}

#[tauri::command]
async fn select_pack(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.activate(&id)
}

#[tauri::command]
async fn import_pack(state: State<'_, AppState>, path: String) -> Result<String, String> {
    let entry = state
        .library
        .import(Path::new(&path))
        .map_err(|error| error.to_string())?;
    Ok(entry.id)
}

#[tauri::command]
async fn delete_pack(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .library
        .remove(&id)
        .map_err(|error| error.to_string())?;
    if state.selected_id() == id {
        state.activate(builtin::default_pack().id)?;
    }
    Ok(())
}

fn activate_initial(library: &Library, settings: &SettingsStore, audio: &AudioEngine) -> String {
    let preferred = settings.load().selected_pack;
    let fallback = builtin::default_pack().id.to_string();
    for id in preferred.into_iter().chain([fallback]) {
        if let Ok(pack) = library.load(&id) {
            audio.set_pack(pack);
            return id;
        }
    }
    unreachable!("組み込みのサウンドパックを読み込めません")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .menu(|app| {
            let app_menu = SubmenuBuilder::new(app, "Khorus")
                .quit_with_text("Khorusを終了")
                .build()?;
            Menu::with_items(app, &[&app_menu])
        })
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let library = Library::new(data_dir.join("packs"))?;
            let settings = SettingsStore::new(data_dir.join("settings.json"));
            let audio = AudioEngine::spawn();
            let selected = activate_initial(&library, &settings, &audio);

            let enabled = Arc::new(AtomicBool::new(true));
            let listener_enabled = Arc::clone(&enabled);
            let listener_audio = audio.clone();
            keyboard::spawn(move |key| {
                if listener_enabled.load(Ordering::SeqCst) {
                    listener_audio.play(key);
                }
            });

            app.manage(AppState {
                enabled,
                audio,
                library,
                settings,
                selected: Mutex::new(selected),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_enabled,
            request_permission,
            list_packs,
            select_pack,
            import_pack,
            delete_pack
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
