mod appearance;
mod archive;
mod audio;
mod builtin;
mod i18n;
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
use tauri::menu::{Menu, SubmenuBuilder};
use tauri::{AppHandle, Manager, State, Wry};
use tauri_plugin_autostart::MacosLauncher;

use crate::audio::AudioEngine;
use crate::i18n::{Language, Locale};
use crate::library::Library;
use crate::settings::SettingsStore;
use crate::tray::TrayMenu;
use crate::volume::Volumes;

const AUTOSTART_ARG: &str = "--autostart";

struct AppState {
    enabled: Arc<AtomicBool>,
    play_on_repeat: Arc<AtomicBool>,
    audio: AudioEngine,
    library: Library,
    settings: SettingsStore,
    tray: TrayMenu,
}

impl AppState {
    fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        let _ = self.tray.toggle.set_checked(enabled);
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
fn get_accent_color() -> Option<String> {
    appearance::accent_color()
}

#[derive(Serialize)]
struct LanguageState {
    language: Language,
    locale: Locale,
}

#[tauri::command]
fn get_language(state: State<AppState>) -> LanguageState {
    LanguageState {
        language: state.settings.get().language,
        locale: i18n::locale(),
    }
}

#[tauri::command]
fn set_language(
    app: AppHandle,
    state: State<AppState>,
    language: Language,
) -> Result<LanguageState, String> {
    i18n::set_locale(language.resolve());
    state.tray.relabel().map_err(|error| error.to_string())?;
    app.set_menu(build_app_menu(&app).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;
    state
        .settings
        .update(|settings| settings.language = language)
        .map_err(|error| error.to_string())?;
    Ok(LanguageState {
        language,
        locale: i18n::locale(),
    })
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
fn get_play_on_repeat(state: State<AppState>) -> bool {
    state.play_on_repeat.load(Ordering::SeqCst)
}

#[tauri::command]
fn set_play_on_repeat(state: State<AppState>, enabled: bool) -> Result<(), String> {
    state.play_on_repeat.store(enabled, Ordering::SeqCst);
    state
        .settings
        .update(|settings| settings.play_on_repeat = enabled)
        .map_err(|error| error.to_string())
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

fn build_app_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let app_menu = SubmenuBuilder::new(app, "Khorus")
        .quit_with_text(i18n::messages().quit_app)
        .build()?;
    Menu::with_items(app, &[&app_menu])
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
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let library = Library::new(data_dir.join("packs"))?;
            let settings = SettingsStore::load(data_dir.join("settings.json"));
            i18n::set_locale(settings.get().language.resolve());
            app.set_menu(build_app_menu(app.handle())?)?;
            let audio = AudioEngine::spawn();
            activate_initial(&library, &settings, &audio);

            let enabled = Arc::new(AtomicBool::new(true));
            let tray = tray::build(app.handle(), true)?;
            let play_on_repeat = Arc::new(AtomicBool::new(settings.get().play_on_repeat));
            let listener_enabled = Arc::clone(&enabled);
            let listener_play_on_repeat = Arc::clone(&play_on_repeat);
            let listener_audio = audio.clone();
            keyboard::spawn(move |key, is_repeat| {
                if !listener_enabled.load(Ordering::SeqCst) {
                    return;
                }
                if is_repeat && !listener_play_on_repeat.load(Ordering::SeqCst) {
                    return;
                }
                listener_audio.play(key);
            });

            if std::env::args().any(|arg| arg == AUTOSTART_ARG) {
                tray::hide_main_window(app.handle());
            }

            app.manage(AppState {
                enabled,
                play_on_repeat,
                audio,
                library,
                settings,
                tray,
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
            get_accent_color,
            get_language,
            set_language,
            get_status,
            set_enabled,
            get_play_on_repeat,
            set_play_on_repeat,
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
