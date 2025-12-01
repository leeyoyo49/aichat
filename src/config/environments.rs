use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::{System, Disks}; 
// 記得先執行 `cargo add sysinfo`

/// ================================
///  Enum 定義
/// ================================
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum OSKind {
    MacOS,
    Linux,
    Windows,
    WSL,
    #[default]
    Unknown,
}

impl std::fmt::Display for OSKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum ShellKind {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
    Msys,
    #[default]
    Unknown,
}

impl std::fmt::Display for ShellKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    Brew,
    Apt,
    Pacman,
    Nix,
    Choco,
    Scoop,
    Winget,
    #[default]
    Unknown,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// =====================================
///  EnvProfile：主要的環境資訊結構
/// =====================================
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvProfile {
    pub os: OSKind,
    pub shell: ShellKind,
    pub pkg: PackageManager,

    // system info
    pub cpu_cores: usize,
    pub cpu_usage: f32, // 整體 CPU 使用率
    pub memory_total_gb: u64, // 改成 GB 比較直觀
    pub memory_used_gb: u64,
    pub disk_total_gb: u64,
    pub disk_available_gb: u64,
    pub gpu_name: Option<String>,
}

impl EnvProfile {
    pub fn detect() -> Self {
        let os = detect_os();
        let shell = detect_shell(&os);
        let pkg = detect_pkg(&os);

        // 偵測硬體資訊
        let (cpu_cores, cpu_usage, mem_total, mem_used, disk_total, disk_avail) =
            detect_system_info();
        
        // 偵測 GPU (可能稍微耗時，但比測網速快得多)
        let gpu_name = detect_gpu();

        Self {
            os,
            shell,
            pkg,
            cpu_cores,
            cpu_usage,
            memory_total_gb: mem_total,
            memory_used_gb: mem_used,
            disk_total_gb: disk_total,
            disk_available_gb: disk_avail,
            gpu_name,
        }
    }

    /// 提供給 AI 的 JSON context
    pub fn to_prompt_context(&self) -> String {
        format!(
r#"<user_environment>
{{
  "os": "{}",
  "shell": "{}",
  "package_manager": "{}",
  "cpu_cores": {},
  "memory_total_gb": {},
  "disk_available_gb": {},
  "gpu_name": "{}"
}}
</user_environment>"#,
            self.os,
            self.shell,
            self.pkg,
            self.cpu_cores,
            self.memory_total_gb,
            self.disk_available_gb,
            self.gpu_name.clone().unwrap_or_else(|| "Unknown".to_string()),
        )
    }
}

/// ================================
///  OS 偵測
/// ================================
fn detect_os() -> OSKind {
    match env::consts::OS {
        "macos" => OSKind::MacOS,
        "linux" => {
            if is_wsl_safe() {
                OSKind::WSL
            } else {
                OSKind::Linux
            }
        }
        "windows" => OSKind::Windows,
        _ => OSKind::Unknown,
    }
}

fn is_wsl_safe() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|content| content.to_lowercase().contains("microsoft"))
        .unwrap_or(false)
}

/// ================================
///  Shell 偵測
/// ================================
fn detect_shell(os: &OSKind) -> ShellKind {
    match os {
        OSKind::Windows => {
            if env::var("PSModulePath").is_ok() {
                return ShellKind::PowerShell;
            }
            if env::var("MSYSTEM").is_ok() || env::var("MINGW_PREFIX").is_ok() {
                return ShellKind::Msys;
            }
            if let Ok(comspec) = env::var("ComSpec") {
                if comspec.to_lowercase().ends_with("cmd.exe") {
                    return ShellKind::Cmd;
                }
            }
            ShellKind::Unknown
        }

        OSKind::Linux | OSKind::WSL | OSKind::MacOS => {
            if let Ok(shell) = env::var("SHELL") {
                let s = shell.to_lowercase();
                if s.contains("bash") { return ShellKind::Bash; }
                if s.contains("zsh") { return ShellKind::Zsh; }
                if s.contains("fish") { return ShellKind::Fish; }
            }
            ShellKind::Unknown
        }

        _ => ShellKind::Unknown,
    }
}

/// ================================
///  Package Manager 偵測
/// ================================
fn detect_pkg(os: &OSKind) -> PackageManager {
    // 使用 which crate (v8.0.0)
    match os {
        OSKind::MacOS => {
            if which::which("brew").is_ok() { return PackageManager::Brew; }
        }
        OSKind::Linux | OSKind::WSL => {
            if which::which("apt-get").is_ok() { return PackageManager::Apt; }
            if which::which("pacman").is_ok() { return PackageManager::Pacman; }
            if which::which("nix").is_ok() { return PackageManager::Nix; }
        }
        OSKind::Windows => {
            if which::which("choco").is_ok() { return PackageManager::Choco; }
            if which::which("scoop").is_ok() { return PackageManager::Scoop; }
            if which::which("winget").is_ok() { return PackageManager::Winget; }
        }
        _ => {}
    }
    PackageManager::Unknown
}

/// ================================
///  System Info 偵測（CPU / RAM / Disk）
/// ================================
fn detect_system_info() -> (usize, f32, u64, u64, u64, u64) {
    // 建立 System 物件但不載入所有資訊以節省時間
    let mut sys = System::new();
    
    // 只重新整理 CPU 和 Memory
    sys.refresh_cpu_all();
    sys.refresh_memory();

    // CPU
    let cores = sys.cpus().len();
    let cpu_usage = sys.global_cpu_usage();

    // Memory (Convert to GB)
    let to_gb = |kb: u64| kb / 1024 / 1024 / 1024;
    let mem_total = to_gb(sys.total_memory());
    let mem_used = to_gb(sys.used_memory());

    // Disks
    let disks = Disks::new_with_refreshed_list();
    // 嘗試找根目錄或第一個硬碟
    let disk = disks.iter().find(|d| d.mount_point() == std::path::Path::new("/")).or_else(|| disks.iter().next());

    let (disk_total, disk_avail) = match disk {
        Some(disk) => (
            to_gb(disk.total_space()),
            to_gb(disk.available_space()),
        ),
        None => (0, 0),
    };

    (cores, cpu_usage, mem_total, mem_used, disk_total, disk_avail)
}

/// ================================
///  GPU 偵測（跨平台）
/// ================================
fn detect_gpu() -> Option<String> {
    let os = env::consts::OS;

    match os {
        "macos" => Some("Apple Silicon / Integrated".to_string()),
        "linux" => {
            // 嘗試 nvidia-smi
            if let Ok(output) = std::process::Command::new("nvidia-smi")
                .args(&["--query-gpu=name", "--format=csv,noheader"])
                .output() 
            {
                if output.status.success() {
                    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !name.is_empty() { return Some(name); }
                }
            }
            // Fallback to lspci (需要 pciutils)
            if let Ok(output) = std::process::Command::new("lspci").output() {
                 let stdout = String::from_utf8_lossy(&output.stdout);
                 for line in stdout.lines() {
                     if line.contains("VGA") || line.contains("3D") {
                         // 簡單擷取顯卡型號
                         let parts: Vec<&str> = line.split(':').collect();
                         if parts.len() > 2 {
                             return Some(parts[2].trim().to_string());
                         }
                     }
                 }
            }
            None
        }
        "windows" => {
            if let Ok(output) = std::process::Command::new("wmic")
                .args(&["path", "win32_VideoController", "get", "name"])
                .output()
            {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<_> = text.lines().skip(1)
                        .map(|l| l.trim())
                        .filter(|l| !l.is_empty())
                        .collect();
                    if !lines.is_empty() { return Some(lines.join(", ")); }
                }
            }
            None
        }
        _ => None,
    }
}