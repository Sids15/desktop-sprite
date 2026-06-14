// Desktop Pet — Tauri backend.
// Owns two windows: the floating "main" pet and the "control" panel.
// Plus the system tray, autostart, persisted config store, and window helpers.
// Animation/state logic lives in the frontend (see ../../src).

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, PhysicalPosition, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

/// Show the floating pet window.
#[tauri::command]
fn show_pet(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Hide the floating pet window (still alive in the tray).
#[tauri::command]
fn hide_pet(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

/// Open (show + focus) the control panel window.
#[tauri::command]
fn open_control(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("control") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Move the pet window to an absolute physical position.
#[tauri::command]
fn set_pet_position(app: tauri::AppHandle, x: i32, y: i32) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_position(PhysicalPosition::new(x, y));
    }
}

/// Center the pet window on the active monitor.
#[tauri::command]
fn center_pet(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.center();
    }
}

/// Read the pet window's current physical position as [x, y].
#[tauri::command]
fn get_pet_position(app: tauri::AppHandle) -> Result<(i32, i32), String> {
    let w = app
        .get_webview_window("main")
        .ok_or("pet window not found")?;
    let p = w.outer_position().map_err(|e| e.to_string())?;
    Ok((p.x, p.y))
}

/// Quit the whole app.
#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            show_pet,
            hide_pet,
            open_control,
            set_pet_position,
            center_pet,
            get_pet_position,
            quit_app
        ])
        .setup(|app| {
            // Closing the control panel hides it to the tray instead of
            // destroying it, so it can be reopened from the tray.
            if let Some(control) = app.get_webview_window("control") {
                let handle = control.clone();
                control.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = handle.hide();
                    }
                });
            }

            // --- System tray + native menu ---
            let open = MenuItem::with_id(app, "open", "Open Control Panel", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Pet", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Pet", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &show, &hide, &quit])?;

            TrayIconBuilder::with_id("pet-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Desktop Pet")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "open" => {
                        if let Some(w) = app.get_webview_window("control") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                        }
                    }
                    "hide" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // Left click on the tray icon opens the control panel.
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("control") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
