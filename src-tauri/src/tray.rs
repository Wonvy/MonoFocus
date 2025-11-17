use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn create_tray() -> SystemTray {
    let enable = CustomMenuItem::new("toggle".to_string(), "Disable Shield");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Exit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(enable)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "toggle" => {
                // 切换护眼模式
                app.emit_all("toggle-shield", ()).unwrap();
            }
            "settings" => {
                // 显示主窗口
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                // 退出应用
                app.exit(0);
            }
            _ => {}
        },
        SystemTrayEvent::LeftClick { .. } => {
            // 左键点击显示主窗口
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    }
}

/// 更新托盘菜单项文本
pub fn update_tray_menu_text(app: &AppHandle, enabled: bool) {
    if let Some(tray) = app.tray_handle().get_item("toggle") {
        let text = if enabled {
            "Disable Shield"
        } else {
            "Enable Shield"
        };
        let _ = tray.set_title(text);
    }
}

