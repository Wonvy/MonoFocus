use crate::monitor::MonitorInfo;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 鼠标位置
#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

/// 获取当前鼠标位置
pub fn get_mouse_position() -> Option<MousePosition> {
    #[cfg(target_os = "windows")]
    return get_mouse_position_windows();
    
    #[cfg(target_os = "macos")]
    return get_mouse_position_macos();
    
    #[cfg(target_os = "linux")]
    return get_mouse_position_linux();
}

/// 判断鼠标在哪个显示器上
pub fn find_monitor_at_position(monitors: &[MonitorInfo], pos: MousePosition) -> Option<String> {
    for monitor in monitors {
        if pos.x >= monitor.x
            && pos.x < monitor.x + monitor.width
            && pos.y >= monitor.y
            && pos.y < monitor.y + monitor.height
        {
            return Some(monitor.id.clone());
        }
    }
    None
}

/// 启动鼠标监听线程
pub fn start_mouse_watcher<F>(callback: F) -> thread::JoinHandle<()>
where
    F: Fn(String) + Send + 'static,
{
    thread::spawn(move || {
        let mut last_monitor_id: Option<String> = None;
        
        loop {
            if let Some(pos) = get_mouse_position() {
                let monitors = crate::monitor::get_monitors();
                
                if let Some(current_monitor_id) = find_monitor_at_position(&monitors, pos) {
                    // 只有当显示器变化时才触发回调
                    if last_monitor_id.as_ref() != Some(&current_monitor_id) {
                        callback(current_monitor_id.clone());
                        last_monitor_id = Some(current_monitor_id);
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(100)); // 100ms 轮询间隔
        }
    })
}

#[cfg(target_os = "windows")]
fn get_mouse_position_windows() -> Option<MousePosition> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point).as_bool() {
            return Some(MousePosition {
                x: point.x,
                y: point.y,
            });
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn get_mouse_position_macos() -> Option<MousePosition> {
    use core_graphics::event::{CGEvent, CGEventType};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
    
    unsafe {
        if let Some(event_source) = CGEventSource::new(CGEventSourceStateID::CombinedSessionState) {
            if let Some(event) = CGEvent::new(event_source) {
                let location = event.location();
                return Some(MousePosition {
                    x: location.x as i32,
                    y: location.y as i32,
                });
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn get_mouse_position_linux() -> Option<MousePosition> {
    use x11::xlib::*;
    use std::ptr;
    
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            return None;
        }
        
        let mut root_return: u64 = 0;
        let mut child_return: u64 = 0;
        let mut root_x: i32 = 0;
        let mut root_y: i32 = 0;
        let mut win_x: i32 = 0;
        let mut win_y: i32 = 0;
        let mut mask_return: u32 = 0;
        
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        
        let result = XQueryPointer(
            display,
            root,
            &mut root_return,
            &mut child_return,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask_return,
        );
        
        XCloseDisplay(display);
        
        if result != 0 {
            return Some(MousePosition {
                x: root_x,
                y: root_y,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mouse_position() {
        let pos = get_mouse_position();
        assert!(pos.is_some());
    }

    #[test]
    fn test_find_monitor_at_position() {
        let monitors = vec![
            MonitorInfo {
                id: "1".to_string(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                physical_width_mm: None,
                physical_height_mm: None,
                scale_factor: 1.0,
            },
        ];
        
        let pos = MousePosition { x: 100, y: 100 };
        let result = find_monitor_at_position(&monitors, pos);
        assert_eq!(result, Some("1".to_string()));
    }
}

