// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod monitor;
mod mouse_watcher;
mod overlay;
mod tray;

use config::{AppConfig, ConfigManager};
use monitor::{get_monitors, normalize_layout, MonitorInfo, UIRect};
use mouse_watcher::{start_mouse_watcher, get_mouse_position, find_monitor_at_position};
use overlay::{OverlayConfig, OverlayManager};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

// 应用状态
struct AppState {
    config_manager: Arc<Mutex<ConfigManager>>,
    overlay_manager: Arc<Mutex<Option<OverlayManager>>>,
    current_monitor_id: Arc<Mutex<Option<String>>>,
}

// Tauri 命令：获取所有显示器信息
#[tauri::command]
fn get_monitor_info() -> Vec<MonitorInfo> {
    get_monitors()
}

// Tauri 命令：获取 UI 布局数据
#[tauri::command]
fn get_monitor_layout(container_width: f32, container_height: f32) -> Vec<UIRect> {
    let monitors = get_monitors();
    normalize_layout(&monitors, container_width, container_height)
}

// Tauri 命令：获取当前配置
#[tauri::command]
fn get_config(state: State<AppState>) -> AppConfig {
    let manager = state.config_manager.lock().unwrap();
    manager.load()
}

// Tauri 命令：更新透明度
#[tauri::command]
fn update_opacity(opacity: f32, state: State<AppState>) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager
        .update_opacity(opacity)
        .map_err(|e| e.to_string())?;

    // 更新遮罩层
    if let Some(overlay_manager) = state.overlay_manager.lock().unwrap().as_ref() {
        let mut config = overlay_manager.get_config();
        config.opacity = opacity;
        overlay_manager.update_config(config);
    }

    Ok(())
}

// Tauri 命令：更新启用状态
#[tauri::command]
fn update_enabled(enabled: bool, state: State<AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    let config = manager.load();
    manager
        .update_enabled(enabled)
        .map_err(|e| e.to_string())?;

    // 更新遮罩层
    if let Some(overlay_manager) = state.overlay_manager.lock().unwrap().as_ref() {
        let mut overlay_config = overlay_manager.get_config();
        overlay_config.enabled = enabled;
        overlay_manager.update_config(overlay_config);
    }

    // 更新托盘菜单（包含语言）
    tray::update_tray_menu_text(&app, enabled, &config.language);

    Ok(())
}

// Tauri 命令：更新自启动
#[tauri::command]
fn update_auto_start(auto_start: bool, state: State<AppState>) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager
        .update_auto_start(auto_start)
        .map_err(|e| e.to_string())
}

// Tauri 命令：更新动画时长
#[tauri::command]
fn update_animation_duration(duration: u64, state: State<AppState>) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager
        .update_animation_duration(duration)
        .map_err(|e| e.to_string())?;

    // 更新遮罩层配置
    if let Some(overlay_manager) = state.overlay_manager.lock().unwrap().as_ref() {
        let mut config = overlay_manager.get_config();
        config.animation_duration = duration;
        overlay_manager.update_config(config);
    }

    Ok(())
}

// Tauri 命令：更新语言
#[tauri::command]
fn update_language(language: String, state: State<AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager
        .update_language(language.clone())
        .map_err(|e| e.to_string())?;
    
    // 重新加载配置以获取最新的 enabled 状态
    let config = manager.load();
    
    // 更新托盘菜单语言
    tray::update_tray_menu_text(&app, config.enabled, &language);
    
    Ok(())
}

// Tauri 命令：获取当前鼠标所在的显示器
#[tauri::command]
fn get_current_monitor(state: State<AppState>) -> Option<String> {
    state.current_monitor_id.lock().unwrap().clone()
}

fn main() {
    // 初始化配置管理器
    let config_manager = Arc::new(Mutex::new(
        ConfigManager::new().expect("Failed to initialize config manager"),
    ));

    let current_monitor_id = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 当尝试启动第二个实例时，显示主窗口
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .system_tray(tray::create_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .setup(move |app| {
            // 初始化遮罩管理器
            let overlay_manager = OverlayManager::new(app.handle());

            // 加载配置并应用
            let config = config_manager.lock().unwrap().load();
            overlay_manager.update_config(OverlayConfig {
                opacity: config.opacity,
                enabled: config.enabled,
                animation_duration: config.animation_duration,
            });

            // 初始化托盘菜单文本
            tray::update_tray_menu_text(&app.handle(), config.enabled, &config.language);

            let overlay_manager = Arc::new(Mutex::new(Some(overlay_manager)));

            // 设置应用状态
            app.manage(AppState {
                config_manager: config_manager.clone(),
                overlay_manager: overlay_manager.clone(),
                current_monitor_id: current_monitor_id.clone(),
            });

            // 初始化遮罩：根据当前鼠标位置设置初始状态
            let monitors = get_monitors();
            if let Some(mouse_pos) = get_mouse_position() {
                if let Some(initial_monitor_id) = find_monitor_at_position(&monitors, mouse_pos) {
                    // 设置初始显示器 ID
                    {
                        let mut current = current_monitor_id.lock().unwrap();
                        *current = Some(initial_monitor_id.clone());
                    }
                    // 初始化遮罩状态
                    if let Some(manager) = overlay_manager.lock().unwrap().as_ref() {
                        manager.update_overlays(&monitors, &initial_monitor_id);
                    }
                }
            }

            // 启动鼠标监听
            let app_handle = app.handle();
            let overlay_manager_clone = overlay_manager.clone();
            let current_monitor_clone = current_monitor_id.clone();

            start_mouse_watcher(move |monitor_id| {
                // 更新当前显示器 ID
                {
                    let mut current = current_monitor_clone.lock().unwrap();
                    *current = Some(monitor_id.clone());
                }

                // 更新遮罩层
                let monitors = get_monitors();
                if let Some(manager) = overlay_manager_clone.lock().unwrap().as_ref() {
                    manager.update_overlays(&monitors, &monitor_id);
                }

                // 通知前端
                let _ = app_handle.emit_all("monitor-changed", monitor_id);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_monitor_info,
            get_monitor_layout,
            get_config,
            update_opacity,
            update_enabled,
            update_auto_start,
            update_animation_duration,
            update_language,
            get_current_monitor,
        ])
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                // 阻止关闭，改为隐藏
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

