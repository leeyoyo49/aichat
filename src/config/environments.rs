use serde::{Deserialize, Serialize};
use std::env;

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
}

impl EnvProfile {
    pub fn detect() -> Self {
        let os = detect_os();
        let shell = detect_shell(&os);
        let pkg = detect_pkg(&os);

        Self { os, shell, pkg }
    }

    /// JSON prompt context（AI 讀得最好）
    pub fn to_prompt_context(&self) -> String {
        format!(
r#"<user_environment>
{{
  "os": "{os}",
  "shell": "{shell}",
  "package_manager": "{pkg}"
}}
</user_environment>"#,
            os = self.os,
            shell = self.shell,
            pkg = self.pkg
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
    match std::fs::read_to_string("/proc/version") {
        Ok(content) => content.to_lowercase().contains("microsoft"),
        Err(_) => false,
    }
}

/// ================================
///  Shell 偵測（避免重複 detect_os）
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
                if s.contains("zsh")  { return ShellKind::Zsh; }
                if s.contains("fish") { return ShellKind::Fish; }
            }

            if matches!(os, OSKind::Linux | OSKind::WSL) {
                if let Ok(name) = std::fs::read_to_string("/proc/self/comm") {
                    let n = name.trim().to_lowercase();
                    if n.contains("bash") { return ShellKind::Bash; }
                    if n.contains("zsh")  { return ShellKind::Zsh; }
                    if n.contains("fish") { return ShellKind::Fish; }
                }
            }

            ShellKind::Unknown
        }

        _ => ShellKind::Unknown,
    }
}

/// ================================
///  Package Manager 偵測（避免重複 detect_os）
/// ================================
fn detect_pkg(os: &OSKind) -> PackageManager {
    match os {
        OSKind::MacOS => {
            if which::which("brew").is_ok() {
                return PackageManager::Brew;
            }
        }

        OSKind::Linux | OSKind::WSL => {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                let s = content.to_lowercase();
                if s.contains("arch") { return PackageManager::Pacman; }
                if s.contains("ubuntu") || s.contains("debian") {
                    return PackageManager::Apt;
                }
                if s.contains("nixos") { return PackageManager::Nix; }
            }

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