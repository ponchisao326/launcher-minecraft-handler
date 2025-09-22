# Minecraft Launcher Backup Library

This Rust library provides an easy-to-use backup system for Minecraft launchers or related applications. It allows users to back up important Minecraft folders (such as saves, mods, configs, screenshots, logs, backups), generate metadata for each backup, exclude specific file extensions, and optionally compress backups into ZIP files with metadata included.

## Features

- **Customizable Backup Options:** Select which Minecraft folders to back up, set output paths, toggle compression, and exclude certain file extensions.
- **Automatic Metadata Generation:** Each backup generates a JSON file containing timestamp, size, file count, and backup options for tracking and auditing.
- **ZIP Compression:** Optionally compress backups into a single ZIP file, including the backup metadata.
- **Easy Integration:** Designed for Rust-based Minecraft launchers or any application needing robust backup functionality.

## Example Usage

```rust
use launcher_minecraft_handler::*;

fn main() {
    let mut backup_options = BackUpOptions::new(
        String::from("C:/Users/Usuario/AppData/Roaming/.minecraft"),
        vec![
            Folders::Saves,
            Folders::Mods,
            Folders::Config,
            Folders::Logs,
            Folders::Screenshots,
            Folders::Backups
        ],
        String::from("D:/MinecraftBackups"),
        true // Enable ZIP compression
    );

    // List all files included in the backup
    let files = backup_options.get_all_files();
    for file in files {
        println!("{}", file);
    }

    // Get total backup size
    let size: u64 = backup_options.get_backup_size();

    // Generate metadata JSON for the backup
    let mut backup_data: BackUpData = BackUpData::new(backup_options.clone(), size);
    backup_data.create_json_file().unwrap();
    println!("Backup data created at: {:?}, with size: {}", backup_data.timestamp, backup_data.size_in_bytes + backup_data.json_size_in_bytes);

    // Create ZIP backup (includes metadata)
    BackupManager::zip_backup(&backup_options);
}
```

## Main Structures

- `Folders`: Enum representing Minecraft folders available for backup (`Saves`, `Config`, `Screenshots`, `Mods`, `Logs`, `Backups`).
- `BackUpOptions`: Configuration for the backup (paths, folders, compression, exclusions).
- `BackUpData`: Metadata structure for each backup, with methods for generating and saving JSON.
- `BackupManager`: Utility for creating ZIP backups, including metadata.

## How It Works

1. **Select Folders:** Configure which Minecraft folders to back up.
2. **Scan Files:** The library collects all files in those folders, respecting excluded extensions.
3. **Generate Metadata:** Metadata about the backup is created and saved as a JSON file.
4. **Compress Backup:** If enabled, all files and metadata are compressed into a ZIP archive.

## Installation

Add these dependencies to your `Cargo.toml`:

```toml
zip = "0.6"
```

You may need to add other dependencies as required (e.g., for filesystem operations).

## Motivation

Originally developed for my own Minecraft launcher, this library is now public to help others perform safe, customizable backups of Minecraft data using Rust.