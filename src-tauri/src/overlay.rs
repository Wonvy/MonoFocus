use crate::monitor::MonitorInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Window, WindowBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub opacity: f32, // 0.0 - 0.8
    pub enabled: bool,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            opacity: 0.6,
            enabled: true,
        }
    }
}

pub struct OverlayManager {
    app: AppHandle,
    overlays: Arc<Mutex<HashMap<String, Window>>>,
    config: Arc<Mutex<OverlayConfig>>,
}

impl OverlayManager {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            overlays: Arc::new(Mutex::new(HashMap::new())),
            config: Arc::new(Mutex::new(OverlayConfig::default())),
        }
    }

    /// 更新遮罩配置
    pub fn update_config(&self, config: OverlayConfig) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config.clone();
        drop(current_config);

        // 更新所有现有遮罩
        self.update_all_overlays();
    }

    /// 获取当前配置
    pub fn get_config(&self) -> OverlayConfig {
        self.config.lock().unwrap().clone()
    }

    /// 更新遮罩显示（根据当前活跃的显示器）
    pub fn update_overlays(&self, monitors: &[MonitorInfo], active_monitor_id: &str) {
        let config = self.config.lock().unwrap().clone();
        
        if !config.enabled {
            self.hide_all_overlays();
            return;
        }

        let mut overlays = self.overlays.lock().unwrap();

        for monitor in monitors {
            if monitor.id == active_monitor_id {
                // 隐藏活跃显示器的遮罩
                if let Some(window) = overlays.get(&monitor.id) {
                    let _ = window.hide();
                }
            } else {
                // 显示或创建非活跃显示器的遮罩
                if let Some(window) = overlays.get(&monitor.id) {
                    let _ = window.show();
                    self.update_overlay_opacity(window, config.opacity);
                } else {
                    // 创建新遮罩
                    if let Ok(window) = self.create_overlay(monitor, config.opacity) {
                        overlays.insert(monitor.id.clone(), window);
                    }
                }
            }
        }
    }

    /// 创建遮罩窗口
    fn create_overlay(&self, monitor: &MonitorInfo, opacity: f32) -> Result<Window, tauri::Error> {
        let label = format!("overlay_{}", monitor.id);
        
        let window = WindowBuilder::new(
            &self.app,
            label.clone(),
            tauri::WindowUrl::App("overlay.html".into()),
        )
        .title("")
        .decorations(false)
        .resizable(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .transparent(true)
        .position(monitor.x as f64, monitor.y as f64)
        .inner_size(monitor.width as f64, monitor.height as f64)
        .build()?;

        // 设置点击穿透
        #[cfg(target_os = "windows")]
        self.set_click_through_windows(&window)?;

        #[cfg(target_os = "macos")]
        self.set_click_through_macos(&window)?;

        #[cfg(target_os = "linux")]
        self.set_click_through_linux(&window)?;

        // 设置透明度
        self.update_overlay_opacity(&window, opacity);

        Ok(window)
    }

    /// 更新遮罩透明度
    fn update_overlay_opacity(&self, window: &Window, opacity: f32) {
        let js = format!(
            "document.body.style.backgroundColor = 'rgba(0, 0, 0, {})'",
            opacity
        );
        let _ = window.eval(&js);
    }

    /// 隐藏所有遮罩
    pub fn hide_all_overlays(&self) {
        let overlays = self.overlays.lock().unwrap();
        for window in overlays.values() {
            let _ = window.hide();
        }
    }

    /// 更新所有遮罩（透明度等）
    fn update_all_overlays(&self) {
        let config = self.config.lock().unwrap().clone();
        let overlays = self.overlays.lock().unwrap();
        
        for window in overlays.values() {
            if config.enabled {
                let _ = window.show();
                self.update_overlay_opacity(window, config.opacity);
            } else {
                let _ = window.hide();
            }
        }
    }

    /// 清理所有遮罩
    pub fn cleanup(&self) {
        let mut overlays = self.overlays.lock().unwrap();
        for (_, window) in overlays.drain() {
            let _ = window.close();
        }
    }

    #[cfg(target_os = "windows")]
    fn set_click_through_windows(&self, window: &Window) -> Result<(), tauri::Error> {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
        };

        unsafe {
            let hwnd = HWND(window.hwnd()?.0 as isize);
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(
                hwnd,
                GWL_EXSTYLE,
                ex_style | WS_EX_LAYERED.0 as isize | WS_EX_TRANSPARENT.0 as isize,
            );
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn set_click_through_macos(&self, window: &Window) -> Result<(), tauri::Error> {
        use cocoa::appkit::NSWindow;
        use cocoa::base::id;

        unsafe {
            let ns_window = window.ns_window()? as id;
            ns_window.setIgnoresMouseEvents_(cocoa::base::YES);
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn set_click_through_linux(&self, window: &Window) -> Result<(), tauri::Error> {
        use x11::xlib::*;
        use std::ffi::CString;
        use std::ptr;

        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Ok(());
            }

            let xid = window.gtk_window()?.window().unwrap().as_raw() as u64;
            
            // 设置窗口类型为 DOCK（置顶但不获取焦点）
            let atom_name = CString::new("_NET_WM_WINDOW_TYPE").unwrap();
            let atom_value = CString::new("_NET_WM_WINDOW_TYPE_DOCK").unwrap();
            
            let type_atom = XInternAtom(display, atom_name.as_ptr(), 0);
            let dock_atom = XInternAtom(display, atom_value.as_ptr(), 0);
            
            XChangeProperty(
                display,
                xid,
                type_atom,
                XA_ATOM,
                32,
                PropModeReplace,
                &dock_atom as *const u64 as *const u8,
                1,
            );

            XCloseDisplay(display);
        }

        Ok(())
    }
}

