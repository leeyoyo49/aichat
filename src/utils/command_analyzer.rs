use super::*;
use anyhow::Result;
use std::path::PathBuf;

/// Command operation types
#[derive(Debug, Clone, PartialEq)]
pub enum CommandOperation {
    Read,         // cat, less, grep, find
    Write,        // echo >, tee
    Modify,       // sed -i, awk
    Delete,       // rm, rmdir
    Move,         // mv, rename
    Copy,         // cp
    Create,       // touch, mkdir
    Execute,      // sh, bash, python
    Network,      // curl, wget, ssh
    System,       // sudo, systemctl
    Unknown,
}

impl CommandOperation {
    pub fn is_destructive(&self) -> bool {
        matches!(self, CommandOperation::Delete | CommandOperation::Modify)
    }

    pub fn needs_backup(&self) -> bool {
        matches!(
            self,
            CommandOperation::Delete
                | CommandOperation::Modify
                | CommandOperation::Move
                | CommandOperation::Write
        )
    }
}

/// Analyzed command information
#[derive(Debug, Clone)]
pub struct CommandAnalysis {
    pub command: String,
    pub operation: CommandOperation,
    pub affected_files: Vec<PathBuf>,
    pub warnings: Vec<String>,
    pub safety_level: SafetyLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,      // Read-only operations
    Caution,   // Write operations
    Dangerous, // Delete/Modify operations
    Critical,  // System operations with sudo
}

impl CommandAnalysis {
    pub fn analyze(command: &str) -> Self {
        let mut analysis = CommandAnalysis {
            command: command.to_string(),
            operation: CommandOperation::Unknown,
            affected_files: Vec::new(),
            warnings: Vec::new(),
            safety_level: SafetyLevel::Safe,
        };

        // Parse command to identify operation
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return analysis;
        }

        let main_cmd = parts[0];

        // Identify operation type
        analysis.operation = match main_cmd {
            "rm" | "rmdir" => CommandOperation::Delete,
            "mv" | "rename" => CommandOperation::Move,
            "cp" => CommandOperation::Copy,
            "touch" | "mkdir" => CommandOperation::Create,
            "sed" | "awk" if command.contains("-i") => CommandOperation::Modify,
            "cat" | "less" | "more" | "grep" | "find" | "ls" => CommandOperation::Read,
            "echo" if command.contains(">") => CommandOperation::Write,
            "tee" => CommandOperation::Write,
            "curl" | "wget" | "ssh" | "scp" | "rsync" => CommandOperation::Network,
            "sudo" | "systemctl" | "service" => CommandOperation::System,
            "sh" | "bash" | "zsh" | "python" | "node" | "ruby" => CommandOperation::Execute,
            _ => CommandOperation::Unknown,
        };

        // Extract affected files
        analysis.affected_files = extract_file_paths_from_command(command);

        // Determine safety level and warnings
        if command.contains("sudo") || command.contains("rm -rf /") {
            analysis.safety_level = SafetyLevel::Critical;
            analysis.warnings.push(
                "⚠️  CRITICAL: This command requires elevated privileges or affects system files!"
                    .to_string(),
            );
        } else if analysis.operation.is_destructive() {
            analysis.safety_level = SafetyLevel::Dangerous;
            analysis
                .warnings
                .push("⚠️  DANGEROUS: This operation cannot be easily undone!".to_string());
        } else if analysis.operation.needs_backup() {
            analysis.safety_level = SafetyLevel::Caution;
            analysis
                .warnings
                .push("⚠️  CAUTION: This operation will modify files.".to_string());
        }

        // Specific warnings
        if main_cmd == "rm" {
            if command.contains("-rf") || command.contains("-r") {
                analysis
                    .warnings
                    .push("⚠️  Recursive delete - will remove directories and all contents!".to_string());
            }
            if command.contains("*") || command.contains("?") {
                analysis
                    .warnings
                    .push("⚠️  Wildcard pattern - multiple files will be affected!".to_string());
            }
        }

        if main_cmd == "mv" && !analysis.affected_files.is_empty() {
            analysis
                .warnings
                .push("💡 Files will be moved/renamed.".to_string());
        }

        // Add backup suggestion
        if analysis.operation.needs_backup() && !analysis.affected_files.is_empty() {
            analysis
                .warnings
                .push("✓ Backup will be created automatically before execution.".to_string());
        }

        analysis
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=".repeat(60)));
        output.push_str(&format!("📊 Command Analysis\n"));
        output.push_str(&format!("{}\n\n", "=".repeat(60)));

        output.push_str(&format!("Command: {}\n", self.command));
        output.push_str(&format!("Operation: {:?}\n", self.operation));
        output.push_str(&format!("Safety Level: {:?}\n\n", self.safety_level));

        if !self.affected_files.is_empty() {
            output.push_str("Affected Files:\n");
            for (i, file) in self.affected_files.iter().enumerate() {
                let exists = if file.exists() { "✓" } else { "✗" };
                output.push_str(&format!(
                    "  {} [{}] {}\n",
                    i + 1,
                    exists,
                    file.display()
                ));
            }
            output.push('\n');
        }

        if !self.warnings.is_empty() {
            output.push_str("Warnings:\n");
            for warning in &self.warnings {
                output.push_str(&format!("  {}\n", warning));
            }
            output.push('\n');
        }

        output.push_str(&format!("{}\n", "=".repeat(60)));

        output
    }
}

/// Preview command impact
pub fn preview_command_impact(command: &str) -> Result<()> {
    let analysis = CommandAnalysis::analyze(command);
    println!("{}", analysis.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_rm_command() {
        let analysis = CommandAnalysis::analyze("rm -rf /tmp/test");
        assert_eq!(analysis.operation, CommandOperation::Delete);
        assert_eq!(analysis.safety_level, SafetyLevel::Dangerous);
        assert!(!analysis.warnings.is_empty());
    }

    #[test]
    fn test_analyze_cat_command() {
        let analysis = CommandAnalysis::analyze("cat file.txt");
        assert_eq!(analysis.operation, CommandOperation::Read);
        assert_eq!(analysis.safety_level, SafetyLevel::Safe);
    }

    #[test]
    fn test_analyze_sudo_command() {
        let analysis = CommandAnalysis::analyze("sudo rm file.txt");
        assert_eq!(analysis.safety_level, SafetyLevel::Critical);
    }
}
