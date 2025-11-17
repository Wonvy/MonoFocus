use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn create_tray() -> SystemTray {
    let enable = CustomMenuItem::new("toggle".to_string(), "护眼模式：开启");
    let settings = CustomMenuItem::new("settings".to_string(), "设置");
    let check_update = CustomMenuItem::new("check_update".to_string(), "检查更新");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

    let tray_menu = SystemTrayMenu::new()
        .add_item(enable)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(check_update)
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
            "check_update" => {
                // 检查更新
                app.emit_all("check-update", ()).unwrap();
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
    let check_update_tray = app.tray_handle().get_item("check_update");
    let quit_tray = app.tray_handle().get_item("quit");
    
    let (toggle_on, toggle_off, settings_text, check_update_text, quit_text) = match language {
        "en" => ("Eye Care Mode: ON", "Eye Care Mode: OFF", "Settings", "Check for Updates", "Exit"),
        "ja" => ("アイケアモード：オン", "アイケアモード：オフ", "設定", "更新を確認", "終了"),
        "fr" => ("Mode protection: ACTIVÉ", "Mode protection: DÉSACTIVÉ", "Paramètres", "Vérifier les mises à jour", "Quitter"),
        "de" => ("Augenschutzmodus: AN", "Augenschutzmodus: AUS", "Einstellungen", "Nach Updates suchen", "Beenden"),
        "es" => ("Modo protección: ACTIVADO", "Modo protección: DESACTIVADO", "Ajustes", "Buscar actualizaciones", "Salir"),
        _ => ("护眼模式：开启", "护眼模式：关闭", "设置", "检查更新", "退出"), // 默认中文
    };
    
    let toggle_text = if enabled { toggle_on } else { toggle_off };
    let _ = toggle_tray.set_title(toggle_text);
    let _ = settings_tray.set_title(settings_text);
    let _ = check_update_tray.set_title(check_update_text);
    let _ = quit_tray.set_title(quit_text);
}

