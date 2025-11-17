use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub physical_width_mm: Option<f32>,
    pub physical_height_mm: Option<f32>,
    pub scale_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub id: String,
}

/// 获取所有显示器信息
pub fn get_monitors() -> Vec<MonitorInfo> {
    #[cfg(target_os = "windows")]
    return get_monitors_windows();
    
    #[cfg(target_os = "macos")]
    return get_monitors_macos();
    
    #[cfg(target_os = "linux")]
    return get_monitors_linux();
}

/// 将显示器信息标准化为 UI 布局
pub fn normalize_layout(monitors: &[MonitorInfo], container_width: f32, container_height: f32) -> Vec<UIRect> {
    if monitors.is_empty() {
        return Vec::new();
    }

    // 1. 计算包络盒
    let min_x = monitors.iter().map(|m| m.x).min().unwrap();
    let max_x = monitors.iter().map(|m| m.x + m.width).max().unwrap();
    let min_y = monitors.iter().map(|m| m.y).min().unwrap();
    let max_y = monitors.iter().map(|m| m.y + m.height).max().unwrap();
    
    let total_width = (max_x - min_x) as f32;
    let total_height = (max_y - min_y) as f32;
    
    // 2. 计算缩放比例（保持纵横比）
    let margin = 20.0;
    let scale = ((container_width - 2.0 * margin) / total_width)
        .min((container_height - 2.0 * margin) / total_height);
    
    // 3. 映射每个显示器
    monitors.iter().map(|m| {
        UIRect {
            x: (m.x - min_x) as f32 * scale + margin,
            y: (m.y - min_y) as f32 * scale + margin,
            width: m.width as f32 * scale,
            height: m.height as f32 * scale,
            id: m.id.clone(),
        }
    }).collect()
}

#[cfg(target_os = "windows")]
fn get_monitors_windows() -> Vec<MonitorInfo> {
    use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW,
    };
    use std::mem;

    let mut monitors = Vec::new();
    
    unsafe extern "system" fn enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &mut *(lparam.0 as *mut Vec<MonitorInfo>);
        
        let mut info: MONITORINFOEXW = mem::zeroed();
        info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
        
        if GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _).as_bool() {
            let rect = info.monitorInfo.rcMonitor;
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            
            // 生成唯一 ID
            let id = format!("monitor_{}", monitors.len());
            
            monitors.push(MonitorInfo {
                id,
                x: rect.left,
                y: rect.top,
                width,
                height,
                physical_width_mm: None, // Windows API 不易获取物理尺寸
                physical_height_mm: None,
                scale_factor: 1.0, // 简化处理，实际应通过 GetDpiForMonitor 获取
            });
        }
        
        BOOL::from(true)
    }
    
    unsafe {
        let monitors_ptr = &mut monitors as *mut Vec<MonitorInfo> as isize;
        EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(enum_proc),
            LPARAM(monitors_ptr),
        );
    }
    
    monitors
}

#[cfg(target_os = "macos")]
fn get_monitors_macos() -> Vec<MonitorInfo> {
    use core_graphics::display::{CGDisplay, CGDirectDisplayID};
    
    let mut monitors = Vec::new();
    
    unsafe {
        let display_count = CGDisplay::active_displays()
            .map(|displays| displays.len())
            .unwrap_or(0);
        
        if let Ok(displays) = CGDisplay::active_displays() {
            for (idx, display_id) in displays.iter().enumerate() {
                let display = CGDisplay::new(*display_id);
                let bounds = display.bounds();
                
                // 尝试获取物理尺寸（毫米）
                let size = CGDisplay::screen_size(*display_id);
                
                monitors.push(MonitorInfo {
                    id: format!("monitor_{}", idx),
                    x: bounds.origin.x as i32,
                    y: bounds.origin.y as i32,
                    width: bounds.size.width as i32,
                    height: bounds.size.height as i32,
                    physical_width_mm: Some(size.width as f32),
                    physical_height_mm: Some(size.height as f32),
                    scale_factor: 1.0,
                });
            }
        }
    }
    
    monitors
}

#[cfg(target_os = "linux")]
fn get_monitors_linux() -> Vec<MonitorInfo> {
    use x11::xlib::*;
    use x11::xrandr::*;
    use std::ptr;
    
    let mut monitors = Vec::new();
    
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            return monitors;
        }
        
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        
        let resources = XRRGetScreenResourcesCurrent(display, root);
        if resources.is_null() {
            XCloseDisplay(display);
            return monitors;
        }
        
        let num_outputs = (*resources).noutput;
        for i in 0..num_outputs {
            let output = *(*resources).outputs.offset(i as isize);
            let output_info = XRRGetOutputInfo(display, resources, output);
            
            if !output_info.is_null() && (*output_info).connection == 0 {
                let crtc_info = XRRGetCrtcInfo(display, resources, (*output_info).crtc);
                
                if !crtc_info.is_null() {
                    monitors.push(MonitorInfo {
                        id: format!("monitor_{}", i),
                        x: (*crtc_info).x,
                        y: (*crtc_info).y,
                        width: (*crtc_info).width as i32,
                        height: (*crtc_info).height as i32,
                        physical_width_mm: Some((*output_info).mm_width as f32),
                        physical_height_mm: Some((*output_info).mm_height as f32),
                        scale_factor: 1.0,
                    });
                    
                    XRRFreeCrtcInfo(crtc_info);
                }
                
                XRRFreeOutputInfo(output_info);
            }
        }
        
        XRRFreeScreenResources(resources);
        XCloseDisplay(display);
    }
    
    monitors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_layout() {
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
            MonitorInfo {
                id: "2".to_string(),
                x: 1920,
                y: 0,
                width: 1920,
                height: 1080,
                physical_width_mm: None,
                physical_height_mm: None,
                scale_factor: 1.0,
            },
        ];
        
        let ui_rects = normalize_layout(&monitors, 400.0, 200.0);
        assert_eq!(ui_rects.len(), 2);
        assert!(ui_rects[0].x >= 0.0);
        assert!(ui_rects[1].x > ui_rects[0].x);
    }
}

