use tauri::Manager;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallStatus {
    pub rust_installed: bool,
    pub zhtp_downloaded: bool,
    pub ready_to_launch: bool,
}

#[tauri::command]
async fn check_system_requirements() -> Result<InstallStatus, String> {
    let rust_installed = Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let zhtp_downloaded = std::path::Path::new("../Cargo.toml").exists();
    
    Ok(InstallStatus {
        rust_installed,
        zhtp_downloaded,
        ready_to_launch: rust_installed && zhtp_downloaded,
    })
}

#[tauri::command]
async fn install_rust() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-Command", "Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe; .\\rustup-init.exe -y"])
            .output()
            .map_err(|e| format!("Failed to install Rust: {}", e))?;
        
        if output.status.success() {
            Ok("Rust installed successfully! Please restart the installer.".to_string())
        } else {
            Err(format!("Rust installation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("sh")
            .args(["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])
            .output()
            .map_err(|e| format!("Failed to install Rust: {}", e))?;
        
        if output.status.success() {
            Ok("Rust installed successfully! Please restart your terminal and the installer.".to_string())
        } else {
            Err(format!("Rust installation failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
}

#[tauri::command]
async fn setup_zhtp() -> Result<String, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let parent_dir = current_dir.parent()
        .ok_or("Failed to get parent directory")?;
    
    #[cfg(target_os = "windows")]
    let setup_script = parent_dir.join("setup.bat");
    
    #[cfg(not(target_os = "windows"))]
    let setup_script = parent_dir.join("setup.sh");
    
    if !setup_script.exists() {
        return Err("Setup script not found".to_string());
    }
    
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", setup_script.to_str().unwrap()])
        .current_dir(parent_dir)
        .output()
        .map_err(|e| format!("Failed to run setup: {}", e))?;
    
    #[cfg(not(target_os = "windows"))]
    let output = Command::new("bash")
        .arg(setup_script.to_str().unwrap())
        .current_dir(parent_dir)
        .output()
        .map_err(|e| format!("Failed to run setup: {}", e))?;
    
    if output.status.success() {
        Ok("ZHTP setup completed successfully!".to_string())
    } else {
        Err(format!("Setup failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[tauri::command]
async fn launch_zhtp() -> Result<String, String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let parent_dir = current_dir.parent()
        .ok_or("Failed to get parent directory")?;
    
    #[cfg(target_os = "windows")]
    let launch_script = parent_dir.join("launch.bat");
    
    #[cfg(not(target_os = "windows"))]
    let launch_script = parent_dir.join("launch.sh");
    
    if !launch_script.exists() {
        return Err("Launch script not found".to_string());
    }
    
    #[cfg(target_os = "windows")]
    let _child = Command::new("cmd")
        .args(["/C", launch_script.to_str().unwrap()])
        .current_dir(parent_dir)
        .spawn()
        .map_err(|e| format!("Failed to launch ZHTP: {}", e))?;
    
    #[cfg(not(target_os = "windows"))]
    let _child = Command::new("bash")
        .arg(launch_script.to_str().unwrap())
        .current_dir(parent_dir)
        .spawn()
        .map_err(|e| format!("Failed to launch ZHTP: {}", e))?;
    
    Ok("ZHTP launched successfully! Check your browser at http://localhost:4000/browser/".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_system_requirements,
            install_rust,
            setup_zhtp,
            launch_zhtp
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
