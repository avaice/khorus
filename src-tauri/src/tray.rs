use tauri::menu::{CheckMenuItem, CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{AppHandle, Manager, Wry};

use crate::i18n::messages;
use crate::AppState;

const MAIN_WINDOW: &str = "main";

pub struct TrayMenu {
    pub toggle: CheckMenuItem<Wry>,
    open: MenuItem<Wry>,
    quit: MenuItem<Wry>,
}

impl TrayMenu {
    pub fn relabel(&self) -> tauri::Result<()> {
        let messages = messages();
        self.toggle.set_text(messages.sound_enabled)?;
        self.open.set_text(messages.open_app)?;
        self.quit.set_text(messages.quit_app)
    }
}

pub fn build(app: &AppHandle, enabled: bool) -> tauri::Result<TrayMenu> {
    let toggle = CheckMenuItemBuilder::with_id("toggle", messages().sound_enabled)
        .checked(enabled)
        .build(app)?;
    let open = MenuItem::with_id(app, "open", messages().open_app, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", messages().quit_app, true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&toggle, &open, &separator, &quit])?;

    TrayIconBuilder::with_id("main")
        .icon(tauri::include_image!("icons/tray.png"))
        .icon_as_template(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                let state = app.state::<AppState>();
                if let Ok(checked) = state.tray.toggle.is_checked() {
                    state.set_enabled(checked);
                }
            }
            "open" => show_main_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(TrayMenu { toggle, open, quit })
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
