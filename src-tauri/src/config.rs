use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub opacity: f32,       // 0.0 - 0.8
    pub enabled: bool,
    pub auto_start: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "auto".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            opacity: 0.6,
            enabled: true,
            auto_start: false,
            theme: "auto".to_string(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.json");
        
        Ok(Self { config_path })
    }

    /// 获取配置文件目录
    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")?;
            Ok(PathBuf::from(appdata).join("MonoFocus"))
        }

        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")?;
            Ok(PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("MonoFocus"))
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
                Ok(PathBuf::from(config_home).join("MonoFocus"))
            } else {
                let home = std::env::var("HOME")?;
                Ok(PathBuf::from(home).join(".config").join("MonoFocus"))
            }
        }
    }

    /// 加载配置
    pub fn load(&self) -> AppConfig {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                return config;
            }
        }
        
        // 如果加载失败，返回默认配置并保存
        let default_config = AppConfig::default();
        let _ = self.save(&default_config);
        default_config
    }

    /// 保存配置
    pub fn save(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// 更新单个字段
    pub fn update_opacity(&self, opacity: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.load();
        config.opacity = opacity.clamp(0.0, 0.8);
        self.save(&config)
    }

    pub fn update_enabled(&self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.load();
        config.enabled = enabled;
        self.save(&config)
    }

    pub fn update_auto_start(&self, auto_start: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.load();
        config.auto_start = auto_start;
        self.save(&config)?;
        
        // 实际设置开机自启动
        #[cfg(target_os = "windows")]
        self.set_auto_start_windows(auto_start)?;
        
        #[cfg(target_os = "macos")]
        self.set_auto_start_macos(auto_start)?;
        
        #[cfg(target_os = "linux")]
        self.set_auto_start_linux(auto_start)?;
        
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn set_auto_start_windows(&self, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_string_lossy();
        
        if enable {
            Command::new("reg")
                .args(&[
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "MonoFocus",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &exe_path_str,
                    "/f",
                ])
                .output()?;
        } else {
            Command::new("reg")
                .args(&[
                    "delete",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "MonoFocus",
                    "/f",
                ])
                .output()
                .ok(); // 忽略删除不存在的错误
        }
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn set_auto_start_macos(&self, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let plist_path = PathBuf::from(home)
            .join("Library")
            .join("LaunchAgents")
            .join("com.monofocus.app.plist");
        
        if enable {
            let exe_path = std::env::current_exe()?;
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.monofocus.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
                exe_path.to_string_lossy()
            );
            
            fs::create_dir_all(plist_path.parent().unwrap())?;
            fs::write(plist_path, plist_content)?;
        } else {
            fs::remove_file(plist_path).ok();
        }
        
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn set_auto_start_linux(&self, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let autostart_dir = PathBuf::from(home).join(".config").join("autostart");
        let desktop_file = autostart_dir.join("monofocus.desktop");
        
        if enable {
            let exe_path = std::env::current_exe()?;
            let desktop_content = format!(
                r#"[Desktop Entry]
Type=Application
Name=MonoFocus
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true"#,
                exe_path.to_string_lossy()
            );
            
            fs::create_dir_all(&autostart_dir)?;
            fs::write(desktop_file, desktop_content)?;
        } else {
            fs::remove_file(desktop_file).ok();
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.opacity, 0.6);
        assert!(config.enabled);
        assert!(!config.auto_start);
    }

    #[test]
    fn test_serialize_config() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("opacity"));
        assert!(json.contains("enabled"));
    }
}

