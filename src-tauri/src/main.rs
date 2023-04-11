use tauri::Manager;
use tauri::{ CustomMenuItem, Menu, MenuItem, Submenu };
// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
    // 关闭启动视图
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // 展示主视图
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    let submenu_gear = Submenu::new(
        "Gear",
        Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit)
    );

    let close = CustomMenuItem::new("close".to_string(), "Close");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu_customer = Submenu::new("Customer", Menu::new().add_item(close).add_item(quit));
    let menus = Menu::new().add_submenu(submenu_customer).add_submenu(submenu_gear);

    tauri::Builder
        ::default()
        .menu(menus)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "close" => {
                    event.window().close().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        // .invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
