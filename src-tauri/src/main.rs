#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Menu, WindowBuilder, WindowUrl, Submenu, CustomMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_window(app_handle: tauri::AppHandle) -> () {
    println!("Opening window.");
    let _window = WindowBuilder::new(
      &app_handle,
      "external_window",
      WindowUrl::App("external.html".into())
    )
    .on_web_resource_request(|request, response| {
        let uri = request.uri();
        match uri {
            "tauri://localhost/assets/test.js" => {
                println!("test file requested!");
                let pwd = std::env::current_dir().unwrap();
                println!("pwd: {:?}", pwd);
                let mutable_response = response.body_mut();
                match std::fs::read(pwd.parent().unwrap().join("src").join("assets").join("test.js")) {
                    Ok(testfile) => {
                        *mutable_response = testfile;
                        response.set_mimetype(Some(String::from("application/javascript")));
                    },
                    Err(e) => {
                        println!("Failed to read file: {:?}", e);
                    }
                }
            },
            _ => {
                println!("Requested uri: {:?}", uri);
            }
        }
    })
    .build();
}

fn main() {
    tauri::Builder::default()
        .menu(Menu::new().add_submenu(Submenu::new(
            "Main Window Option",
            Menu::new().add_item(CustomMenuItem::new("Hello-from-main", "Hello From Main")),
        )))
        .invoke_handler(tauri::generate_handler![open_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



