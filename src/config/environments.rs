use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::{System, Disks};
use which::which;

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
    pub cpu_usage: f32,
    pub memory_total_kb: u64,
    pub memory_used_kb: u64,
    pub disk_total_kb: u64,
    pub disk_available_kb: u64,
}

impl EnvProfile {
    /// 不使用 async -> config 端不需要改
    pub fn detect() -> Self {
        let os = detect_os();
        let shell = detect_shell(&os);
        let pkg = detect_pkg(&os);

        let (cpu_cores, cpu_usage, mem_total, mem_used, disk_total, disk_avail) =
            detect_system_info();

        Self {
            os,
            shell,
            pkg,
            cpu_cores,
            cpu_usage,
            memory_total_kb: mem_total,
            memory_used_kb: mem_used,
            disk_total_kb: disk_total,
            disk_available_kb: disk_avail,
        }
    }

    /// 提供給 AI 的 JSON-like prompt context
    pub fn to_prompt_context(&self) -> String {
        format!(
r#"<user_environment>
{{
  "os": "{os}",
  "shell": "{shell}",
  "package_manager": "{pkg}",
  "cpu_cores": {cpu_cores},
  "cpu_usage_percent": {cpu_usage},
  "memory_total_kb": {mem_total},
  "memory_used_kb": {mem_used},
  "disk_total_kb": {disk_total},
  "disk_available_kb": {disk_avail}
}}
</user_environment>"#,
            os = self.os,
            shell = self.shell,
            pkg = self.pkg,
            cpu_cores = self.cpu_cores,
            cpu_usage = self.cpu_usage,
            mem_total = self.memory_total_kb,
            mem_used = self.memory_used_kb,
            disk_total = self.disk_total_kb,
            disk_avail = self.disk_available_kb,
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
                if s.contains("bash") {
                    return ShellKind::Bash;
                }
                if s.contains("zsh") {
                    return ShellKind::Zsh;
                }
                if s.contains("fish") {
                    return ShellKind::Fish;
                }
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
    match os {
        OSKind::MacOS => {
            if which("brew").is_ok() {
                return PackageManager::Brew;
            }
        }

        OSKind::Linux | OSKind::WSL => {
            if which("apt-get").is_ok() {
                return PackageManager::Apt;
            }
            if which("pacman").is_ok() {
                return PackageManager::Pacman;
            }
            if which("nix").is_ok() {
                return PackageManager::Nix;
            }
        }

        OSKind::Windows => {
            if which("choco").is_ok() {
                return PackageManager::Choco;
            }
            if which("scoop").is_ok() {
                return PackageManager::Scoop;
            }
            if which("winget").is_ok() {
                return PackageManager::Winget;
            }
        }

        _ => {}
    }

    PackageManager::Unknown
}

/// ================================
///  System Info 偵測（CPU / RAM / Disk）
/// ================================
fn detect_system_info() -> (usize, f32, u64, u64, u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU
    let cores = sys.cpus().len();
    let cpu_usage = sys.global_cpu_usage();

    // Memory
    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();

    // Disks
    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .iter()
        .find(|d| d.mount_point() == "/")
        .or_else(|| disks.iter().next());

    let (disk_total, disk_avail) = match disk {
        Some(disk) => (
            disk.total_space() / 1024,      // KB
            disk.available_space() / 1024, // KB
        ),
        None => (0, 0),
    };

    (cores, cpu_usage, mem_total, mem_used, disk_total, disk_avail)
}