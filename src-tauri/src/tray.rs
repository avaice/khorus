use tauri::menu::{CheckMenuItem, CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{AppHandle, Manager, Wry};

use crate::AppState;

const MAIN_WINDOW: &str = "main";

pub fn build(app: &AppHandle, enabled: bool) -> tauri::Result<CheckMenuItem<Wry>> {
    let toggle = CheckMenuItemBuilder::with_id("toggle", "音を鳴らす")
        .checked(enabled)
        .build(app)?;
    let open = MenuItem::with_id(app, "open", "Khorusを開く", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Khorusを終了", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&toggle, &open, &separator, &quit])?;

    TrayIconBuilder::with_id("main")
        .icon(tauri::include_image!("icons/tray.png"))
        .icon_as_template(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                let state = app.state::<AppState>();
                if let Ok(checked) = state.tray_toggle.is_checked() {
                    state.set_enabled(checked);
                }
            }
            "open" => show_main_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(toggle)
}

pub fn show_main_window(app: &AppHandle) {
    set_dock_visible(app, true);
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let _ = window.hide();
    }
    set_dock_visible(app, false);
}

fn set_dock_visible(app: &AppHandle, visible: bool) {
    #[cfg(target_os = "macos")]
    {
        let policy = if visible {
            ActivationPolicy::Regular
        } else {
            ActivationPolicy::Accessory
        };
        let _ = app.set_activation_policy(policy);
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (app, visible);
}
