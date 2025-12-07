use crate::config::{EnvProfile, GlobalConfig};
use anyhow::Result;
use std::collections::HashMap;

/// Command tutorial information
pub struct CommandTutorial {
    pub command: String,
    pub structure: Vec<CommandPart>,
    pub environment_notes: Vec<String>,
    pub safety_notes: Vec<String>,
    pub man_page_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CommandPart {
    pub text: String,
    pub description: String,
    pub part_type: PartType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartType {
    Command,
    Flag,
    Argument,
    Option,
    File,
    Redirect,
}

impl CommandTutorial {
    pub fn analyze(command: &str, env: &EnvProfile) -> Self {
        let mut tutorial = CommandTutorial {
            command: command.to_string(),
            structure: Vec::new(),
            environment_notes: Vec::new(),
            safety_notes: Vec::new(),
            man_page_ref: None,
        };

        // Parse command structure
        tutorial.structure = Self::parse_structure(command);

        // Add environment-specific notes
        tutorial.add_environment_notes(env);

        // Add safety notes
        tutorial.add_safety_notes();

        // Add man page reference
        if let Some(first_part) = tutorial.structure.first() {
            if first_part.part_type == PartType::Command {
                tutorial.man_page_ref = Some(format!("man {}", first_part.text));
            }
        }

        tutorial
    }

    fn parse_structure(command: &str) -> Vec<CommandPart> {
        let mut parts = Vec::new();
        let words: Vec<&str> = command.split_whitespace().collect();

        if words.is_empty() {
            return parts;
        }

        // First word is the command
        let cmd = words[0];
        parts.push(CommandPart {
            text: cmd.to_string(),
            description: Self::get_command_description(cmd),
            part_type: PartType::Command,
        });

        // Parse remaining parts
        let mut i = 1;
        while i < words.len() {
            let word = words[i];

            if word.starts_with('-') {
                // It's a flag or option
                if word.starts_with("--") {
                    // Long option
                    parts.push(CommandPart {
                        text: word.to_string(),
                        description: Self::get_long_option_description(cmd, word),
                        part_type: PartType::Option,
                    });
                } else {
                    // Short flag(s)
                    parts.push(CommandPart {
                        text: word.to_string(),
                        description: Self::get_flag_description(cmd, word),
                        part_type: PartType::Flag,
                    });
                }
            } else if word.contains('>') || word.contains('<') || word.contains('|') {
                // Redirect or pipe
                parts.push(CommandPart {
                    text: word.to_string(),
                    description: Self::get_redirect_description(word),
                    part_type: PartType::Redirect,
                });
            } else if word.contains('/') || word.contains('.') {
                // Likely a file path
                parts.push(CommandPart {
                    text: word.to_string(),
                    description: "File or path".to_string(),
                    part_type: PartType::File,
                });
            } else {
                // Regular argument
                parts.push(CommandPart {
                    text: word.to_string(),
                    description: "Argument".to_string(),
                    part_type: PartType::Argument,
                });
            }

            i += 1;
        }

        parts
    }

    fn get_command_description(cmd: &str) -> String {
        let descriptions: HashMap<&str, &str> = [
            ("ls", "List directory contents"),
            ("cd", "Change directory"),
            ("cp", "Copy files or directories"),
            ("mv", "Move or rename files"),
            ("rm", "Remove files or directories"),
            ("mkdir", "Create directories"),
            ("cat", "Concatenate and display files"),
            ("grep", "Search text using patterns"),
            ("find", "Search for files in directory hierarchy"),
            ("tar", "Archive utility"),
            ("curl", "Transfer data from URLs"),
            ("wget", "Download files from the web"),
            ("git", "Version control system"),
            ("ssh", "Secure shell remote login"),
            ("chmod", "Change file permissions"),
            ("chown", "Change file ownership"),
            ("ps", "Display process status"),
            ("kill", "Terminate processes"),
            ("top", "Display system tasks"),
            ("df", "Display disk space usage"),
            ("du", "Estimate file space usage"),
            ("sed", "Stream editor for filtering and transforming text"),
            ("awk", "Pattern scanning and processing language"),
        ]
        .iter()
        .cloned()
        .collect();

        descriptions
            .get(cmd)
            .unwrap_or(&"Command")
            .to_string()
    }

    fn get_flag_description(cmd: &str, flag: &str) -> String {
        // Common flags for various commands
        match (cmd, flag) {
            ("ls", "-l") => "Long format with detailed information".to_string(),
            ("ls", "-a") => "Include hidden files (those starting with .)".to_string(),
            ("ls", "-h") => "Human-readable file sizes".to_string(),
            ("rm", "-r") | ("rm", "-R") => "Recursive - remove directories and contents".to_string(),
            ("rm", "-f") => "Force - ignore nonexistent files, never prompt".to_string(),
            ("cp", "-r") | ("cp", "-R") => "Recursive - copy directories".to_string(),
            ("cp", "-v") => "Verbose - show files being copied".to_string(),
            ("tar", "-c") => "Create a new archive".to_string(),
            ("tar", "-x") => "Extract files from archive".to_string(),
            ("tar", "-z") => "Compress/decompress with gzip".to_string(),
            ("tar", "-f") => "Specify filename for archive".to_string(),
            ("tar", "-v") => "Verbose output".to_string(),
            ("grep", "-i") => "Case-insensitive search".to_string(),
            ("grep", "-r") => "Recursive search in directories".to_string(),
            ("grep", "-n") => "Show line numbers".to_string(),
            _ => format!("Flag: {}", flag),
        }
    }

    fn get_long_option_description(_cmd: &str, option: &str) -> String {
        // Extract option name
        let opt_name = option.trim_start_matches("--");
        match opt_name {
            "help" => "Display help information".to_string(),
            "version" => "Display version information".to_string(),
            "verbose" => "Verbose output".to_string(),
            "force" => "Force operation without prompting".to_string(),
            "recursive" => "Process directories recursively".to_string(),
            _ => format!("Option: {}", option),
        }
    }

    fn get_redirect_description(word: &str) -> String {
        if word.contains('>') {
            if word.contains(">>") {
                "Append output to file".to_string()
            } else {
                "Redirect output to file (overwrite)".to_string()
            }
        } else if word.contains('<') {
            "Read input from file".to_string()
        } else if word.contains('|') {
            "Pipe output to next command".to_string()
        } else {
            "Redirection".to_string()
        }
    }

    fn add_environment_notes(&mut self, env: &EnvProfile) {
        // Add OS-specific notes
        let os_note = format!("Current OS: {:?}", env.os);
        self.environment_notes.push(os_note);

        let shell_note = format!("Current Shell: {:?}", env.shell);
        self.environment_notes.push(shell_note);

        // Add command-specific environment notes
        if let Some(first_part) = self.structure.first() {
            match first_part.text.as_str() {
                "brew" => {
                    self.environment_notes.push(
                        "✓ Homebrew is the package manager for macOS".to_string(),
                    );
                }
                "apt" | "apt-get" => {
                    self.environment_notes.push(
                        "✓ APT is the package manager for Debian/Ubuntu systems".to_string(),
                    );
                }
                "pacman" => {
                    self.environment_notes.push(
                        "✓ Pacman is the package manager for Arch Linux".to_string(),
                    );
                }
                "yum" | "dnf" => {
                    self.environment_notes.push(
                        "✓ DNF/YUM is the package manager for Fedora/RHEL systems".to_string(),
                    );
                }
                _ => {}
            }
        }
    }

    fn add_safety_notes(&mut self) {
        // Analyze command for safety concerns
        let cmd_lower = self.command.to_lowercase();

        if cmd_lower.contains("rm") {
            if cmd_lower.contains("-rf") || cmd_lower.contains("-r") {
                self.safety_notes
                    .push("⚠️  CAUTION: Recursive delete - will remove all files and subdirectories!".to_string());
            }
            if cmd_lower.contains("*") || cmd_lower.contains("/*") {
                self.safety_notes
                    .push("⚠️  DANGER: Wildcard in rm command - verify which files will be deleted!".to_string());
            }
            self.safety_notes
                .push("💡 Consider using -i flag for interactive prompts before deletion".to_string());
        }

        if cmd_lower.contains("sudo") {
            self.safety_notes
                .push("⚠️  This command requires administrator privileges".to_string());
            self.safety_notes
                .push("💡 Only run sudo commands you fully understand".to_string());
        }

        if cmd_lower.contains("chmod") && cmd_lower.contains("777") {
            self.safety_notes
                .push("⚠️  chmod 777 gives all permissions to everyone - security risk!".to_string());
        }

        // Positive notes for safe commands
        if cmd_lower.starts_with("ls")
            || cmd_lower.starts_with("cat")
            || cmd_lower.starts_with("grep")
        {
            self.safety_notes
                .push("✓ This is a read-only operation - safe to execute".to_string());
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=".repeat(60)));
        output.push_str("📚 Command Tutorial\n");
        output.push_str(&format!("{}\n\n", "=".repeat(60)));

        output.push_str(&format!("Command: {}\n\n", self.command));

        // Structure breakdown
        output.push_str("Structure Breakdown:\n");
        for (_i, part) in self.structure.iter().enumerate() {
            let icon = match part.part_type {
                PartType::Command => "▶️",
                PartType::Flag => "🚩",
                PartType::Option => "⚙️",
                PartType::File => "📄",
                PartType::Argument => "📝",
                PartType::Redirect => "➡️",
            };
            output.push_str(&format!(
                "  {} {:<15} - {}\n",
                icon, part.text, part.description
            ));
        }
        output.push('\n');

        // Environment context
        if !self.environment_notes.is_empty() {
            output.push_str("Environment Context:\n");
            for note in &self.environment_notes {
                output.push_str(&format!("  {}\n", note));
            }
            output.push('\n');
        }

        // Safety notes
        if !self.safety_notes.is_empty() {
            output.push_str("Safety Notes:\n");
            for note in &self.safety_notes {
                output.push_str(&format!("  {}\n", note));
            }
            output.push('\n');
        }

        // Man page reference
        if let Some(man_ref) = &self.man_page_ref {
            output.push_str(&format!("📖 For more details: {}\n\n", man_ref));
        }

        output.push_str(&format!("{}\n", "=".repeat(60)));

        output
    }
}

/// Show command tutorial
pub fn show_command_tutorial(command: &str, _config: &GlobalConfig) -> Result<()> {
    let env = EnvProfile::detect();
    let tutorial = CommandTutorial::analyze(command, &env);
    println!("{}", tutorial.display());
    Ok(())
}
