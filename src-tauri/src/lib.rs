mod archive;
mod audio;
mod builtin;
mod keyboard;
mod keys;
mod library;
mod pack;
mod settings;
mod tray;
mod volume;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::Serialize;
use tauri::menu::{CheckMenuItem, Menu, SubmenuBuilder};
use tauri::{Manager, State, Wry};
use tauri_plugin_autostart::MacosLauncher;

use crate::audio::AudioEngine;
use crate::library::Library;
use crate::settings::SettingsStore;
use crate::volume::Volumes;

const AUTOSTART_ARG: &str = "--autostart";

struct AppState {
    enabled: Arc<AtomicBool>,
    audio: AudioEngine,
    library: Library,
    settings: SettingsStore,
    tray_toggle: CheckMenuItem<Wry>,
}

impl AppState {
    fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        let _ = self.tray_toggle.set_checked(enabled);
    }

    fn selected_id(&self) -> String {
        self.settings
            .get()
            .selected_pack
            .unwrap_or_else(|| builtin::default_pack().id.to_string())
    }

    fn activate(&self, id: &str) -> Result<(), String> {
        let pack = self.library.load(id).map_err(|error| error.to_string())?;
        self.audio.set_pack(pack);
        self.settings
            .update(|settings| settings.selected_pack = Some(id.to_string()))
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
    state.set_enabled(enabled);
}

#[tauri::command]
fn get_volumes(state: State<AppState>) -> Volumes {
    state.settings.get().volumes
}

#[tauri::command]
fn set_volumes(state: State<AppState>, volumes: Volumes) -> Result<(), String> {
    let volumes = volumes.clamped();
    state.audio.set_volumes(volumes);
    state
        .settings
        .update(|settings| settings.volumes = volumes)
        .map_err(|error| error.to_string())
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

fn activate_initial(library: &Library, settings: &SettingsStore, audio: &AudioEngine) {
    audio.set_volumes(settings.get().volumes.clamped());
    let fallback = builtin::default_pack().id.to_string();
    for id in settings.get().selected_pack.into_iter().chain([fallback]) {
        if let Ok(pack) = library.load(&id) {
            audio.set_pack(pack);
            let _ = settings.update(|settings| settings.selected_pack = Some(id));
            return;
        }
    }
    unreachable!("組み込みのサウンドパックを読み込めません")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![AUTOSTART_ARG]),
        ))
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
            let settings = SettingsStore::load(data_dir.join("settings.json"));
            let audio = AudioEngine::spawn();
            activate_initial(&library, &settings, &audio);

            let enabled = Arc::new(AtomicBool::new(true));
            let tray_toggle = tray::build(app.handle(), true)?;
            let listener_enabled = Arc::clone(&enabled);
            let listener_audio = audio.clone();
            keyboard::spawn(move |key| {
                if listener_enabled.load(Ordering::SeqCst) {
                    listener_audio.play(key);
                }
            });

            if std::env::args().any(|arg| arg == AUTOSTART_ARG) {
                tray::hide_main_window(app.handle());
            }

            app.manage(AppState {
                enabled,
                audio,
                library,
                settings,
                tray_toggle,
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                tray::hide_main_window(window.app_handle());
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_enabled,
            get_volumes,
            set_volumes,
            request_permission,
            list_packs,
            select_pack,
            import_pack,
            delete_pack
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                tray::show_main_window(app);
            }
            #[cfg(not(target_os = "macos"))]
            let _ = (app, event);
        });
}
