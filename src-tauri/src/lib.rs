// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{Menu, MenuItem},
    tray::{ TrayIconBuilder, TrayIconEvent},
    Manager, Window,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            
            let app_handle = app.handle();
            let icon = app.default_window_icon().unwrap().clone();
            
            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .build(app)?;
                
            let app_handle_clone = app_handle.clone();
            tray.on_tray_icon_event(move |_app_handle, event| {
                match event {
                    TrayIconEvent::Click { .. } => {
                        // Single click - show window
                        println!("Single click detected - showing window");
                        if let Some(window) = app_handle_clone.get_webview_window("main") {
                            if window.is_visible().unwrap() {
                                window.hide().unwrap();
                            } else {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                    }
                    TrayIconEvent::DoubleClick { .. } => {
                        // Double click detected - show quit menu
                        println!("Double click detected - showing quit menu");
                        // tray.show_menu().unwrap();
                    }
                    _ => {}
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
