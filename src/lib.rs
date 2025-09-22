use std::time::SystemTime;

#[derive(Debug)]
pub enum Folders {
    Saves,
    Config,
    Screenshots,
    Mods,
    Logs,
    Backups
}

pub struct BackUpOptions {
    pub folder_options: Vec<Folders>,
    pub destination_path: String,
    pub compress: bool,
    pub excluded_extensions: Vec<String>
}

pub struct BackUpData {
    options: BackUpOptions,
    timestamp: SystemTime,
    size_in_bytes: u64,
    file_count: u32
}

pub struct BackupManager;

impl BackUpOptions {
    pub fn new(folder_options: Vec<Folders>, destination_path: String) -> Self {
        BackUpOptions {
            folder_options,
            destination_path,
            compress: true,
            excluded_extensions: Vec::new()
        }
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }

    pub fn add_excluded_extension(&mut self, extension: String) {
        self.excluded_extensions.push(extension);
    }
}

impl BackUpData {
    pub fn new(options: BackUpOptions, size_in_bytes: u64, file_count: u32) -> Self {
        BackUpData {
            options,
            timestamp: SystemTime::now(),
            size_in_bytes,
            file_count
        }
    }

    pub fn format_json(&self) -> String {
        format!(
            r#"{{
                "timestamp": {:?},
                "size_in_bytes": {},
                "file_count": {},
                "options": {{
                    "folder_options": {:?},
                    "destination_path": "{}",
                    "compress": {},
                    "excluded_extensions": {:?}
                }}
            }}"#,
            self.timestamp,
            self.size_in_bytes,
            self.file_count,
            self.options.folder_options,
            self.options.destination_path,
            self.options.compress,
            self.options.excluded_extensions
        )
    }
}