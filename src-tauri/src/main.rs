#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Menu, WindowBuilder, WindowUrl, Submenu, CustomMenuItem, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_window(app_handle: tauri::AppHandle) -> () {
    println!("Opening window.");
    let _window = WindowBuilder::new(
      &app_handle,
      "external_window",
      WindowUrl::App("external.html".into())
    ).menu(Menu::new().add_submenu(Submenu::new(
        "External Window Option",
        Menu::new().add_item(CustomMenuItem::new("Hello-from-external", "Hello From External")),
    ))).build();
}

#[tauri::command]
fn unminimize_window(app_handle: tauri::AppHandle) -> () {
    println!("Attempting to unminimize window.");
    match app_handle.get_window("external_window") {
        Some(window) => window.unminimize().unwrap(),
        None => println!("@unminimize_window: External window is not open."),
    };
}

fn main() {
    tauri::Builder::default()
        .menu(Menu::new().add_submenu(Submenu::new(
            "Main Window Option",
            Menu::new().add_item(CustomMenuItem::new("Hello-from-main", "Hello From Main")),
        )))
        .invoke_handler(tauri::generate_handler![open_window, unminimize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



