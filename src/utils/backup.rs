use anyhow::{anyhow, bail, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

const BACKUP_DIR_NAME: &str = ".aichat_backups";
const BACKUP_INDEX_FILE: &str = "backup_index.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: String,
    pub timestamp: String,
    pub command: String,
    pub files: Vec<BackupFile>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFile {
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub file_hash: String,
}

pub struct BackupManager {
    backup_dir: PathBuf,
    index_file: PathBuf,
}

impl BackupManager {
    pub fn new() -> Result<Self> {
        let backup_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Cannot determine home directory"))?
            .join(BACKUP_DIR_NAME);

        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }

        let index_file = backup_dir.join(BACKUP_INDEX_FILE);

        Ok(Self {
            backup_dir,
            index_file,
        })
    }

    pub fn create_backup(&self, command: &str, paths: Vec<PathBuf>) -> Result<BackupEntry> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = Local::now().to_rfc3339();
        let backup_subdir = self.backup_dir.join(&id);
        fs::create_dir_all(&backup_subdir)?;

        let mut backup_files = Vec::new();

        for path in paths {
            if !path.exists() {
                continue; // Skip non-existent files
            }

            // Only backup if it's a file (not directory for now)
            if path.is_file() {
                let file_name = path
                    .file_name()
                    .ok_or_else(|| anyhow!("Invalid file name"))?;
                let backup_path = backup_subdir.join(file_name);

                // Copy file
                fs::copy(&path, &backup_path)?;

                // Calculate hash for verification
                let file_hash = self.calculate_file_hash(&path)?;

                backup_files.push(BackupFile {
                    original_path: path,
                    backup_path,
                    file_hash,
                });
            }
        }

        let entry = BackupEntry {
            id: id.clone(),
            timestamp,
            command: command.to_string(),
            files: backup_files,
            description: format!("Backup before executing: {}", command),
        };

        // Add to index
        self.add_to_index(&entry)?;

        Ok(entry)
    }

    pub fn restore_backup(&self, backup_id: &str) -> Result<()> {
        let entry = self.get_backup_entry(backup_id)?;

        for file in &entry.files {
            if file.backup_path.exists() {
                // Restore file
                if let Some(parent) = file.original_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&file.backup_path, &file.original_path)?;
                println!(
                    "✓ Restored: {}",
                    file.original_path.display()
                );
            } else {
                eprintln!(
                    "⚠ Backup file not found: {}",
                    file.backup_path.display()
                );
            }
        }

        println!("✓ Backup {} restored successfully", backup_id);
        Ok(())
    }

    pub fn list_backups(&self) -> Result<Vec<BackupEntry>> {
        if !self.index_file.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.index_file)?;
        let reader = BufReader::new(file);
        let entries: HashMap<String, BackupEntry> = serde_json::from_reader(reader)?;

        let mut list: Vec<BackupEntry> = entries.into_values().collect();
        list.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(list)
    }

    pub fn get_backup_entry(&self, backup_id: &str) -> Result<BackupEntry> {
        if !self.index_file.exists() {
            bail!("No backups found");
        }

        let file = File::open(&self.index_file)?;
        let reader = BufReader::new(file);
        let entries: HashMap<String, BackupEntry> = serde_json::from_reader(reader)?;

        entries
            .get(backup_id)
            .cloned()
            .ok_or_else(|| anyhow!("Backup {} not found", backup_id))
    }

    pub fn delete_backup(&self, backup_id: &str) -> Result<()> {
        let _entry = self.get_backup_entry(backup_id)?;

        // Delete backup directory
        let backup_subdir = self.backup_dir.join(backup_id);
        if backup_subdir.exists() {
            fs::remove_dir_all(&backup_subdir)?;
        }

        // Remove from index
        self.remove_from_index(backup_id)?;

        println!("✓ Backup {} deleted", backup_id);
        Ok(())
    }

    pub fn cleanup_old_backups(&self, keep_count: usize) -> Result<()> {
        let mut backups = self.list_backups()?;

        if backups.len() <= keep_count {
            return Ok(());
        }

        // Sort by timestamp (oldest first)
        backups.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let to_delete = backups.len() - keep_count;
        for backup in backups.iter().take(to_delete) {
            self.delete_backup(&backup.id)?;
        }

        println!("✓ Cleaned up {} old backups", to_delete);
        Ok(())
    }

    fn add_to_index(&self, entry: &BackupEntry) -> Result<()> {
        let mut entries: HashMap<String, BackupEntry> = if self.index_file.exists() {
            let file = File::open(&self.index_file)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        } else {
            HashMap::new()
        };

        entries.insert(entry.id.clone(), entry.clone());

        let file = File::create(&self.index_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &entries)?;

        Ok(())
    }

    fn remove_from_index(&self, backup_id: &str) -> Result<()> {
        if !self.index_file.exists() {
            return Ok(());
        }

        let file = File::open(&self.index_file)?;
        let reader = BufReader::new(file);
        let mut entries: HashMap<String, BackupEntry> = serde_json::from_reader(reader)?;

        entries.remove(backup_id);

        let file = File::create(&self.index_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &entries)?;

        Ok(())
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        use sha2::{Digest, Sha256};
        let contents = fs::read(path)?;
        let hash = Sha256::digest(&contents);
        Ok(format!("{:x}", hash))
    }
}

/// Extract file paths from a shell command (basic implementation)
pub fn extract_file_paths_from_command(command: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let words: Vec<&str> = command.split_whitespace().collect();

    for word in words {
        // Skip flags and common commands
        if word.starts_with('-') || is_common_command(word) {
            continue;
        }

        // Check if it looks like a file path
        let path = PathBuf::from(word);
        if path.exists() && path.is_file() {
            paths.push(path);
        }
    }

    paths
}

fn is_common_command(word: &str) -> bool {
    matches!(
        word,
        "ls"
            | "cd"
            | "pwd"
            | "echo"
            | "cat"
            | "grep"
            | "find"
            | "sed"
            | "awk"
            | "rm"
            | "mv"
            | "cp"
            | "mkdir"
            | "touch"
            | "chmod"
            | "chown"
            | "sudo"
            | "curl"
            | "wget"
            | "git"
            | "docker"
            | "npm"
            | "yarn"
            | "cargo"
            | "python"
            | "node"
            | "bash"
            | "sh"
            | "zsh"
    )
}
