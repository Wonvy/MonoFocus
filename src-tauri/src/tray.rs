use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn create_tray() -> SystemTray {
    let enable = CustomMenuItem::new("toggle".to_string(), "护眼模式：开启");
    let settings = CustomMenuItem::new("settings".to_string(), "设置");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

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

/// 更新托盘菜单项文本（根据状态和语言）
pub fn update_tray_menu_text(app: &AppHandle, enabled: bool, language: &str) {
    let toggle_tray = app.tray_handle().get_item("toggle");
    let settings_tray = app.tray_handle().get_item("settings");
    let quit_tray = app.tray_handle().get_item("quit");
    
    if language == "en" {
        // 英文
        let toggle_text = if enabled {
            "Eye Care Mode: ON"
        } else {
            "Eye Care Mode: OFF"
        };
        let _ = toggle_tray.set_title(toggle_text);
        let _ = settings_tray.set_title("Settings");
        let _ = quit_tray.set_title("Exit");
    } else {
        // 中文
        let toggle_text = if enabled {
            "护眼模式：开启"
        } else {
            "护眼模式：关闭"
        };
        let _ = toggle_tray.set_title(toggle_text);
        let _ = settings_tray.set_title("设置");
        let _ = quit_tray.set_title("退出");
    }
}

