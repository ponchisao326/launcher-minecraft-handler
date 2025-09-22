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
    pub default_minecraft_path: String,
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
    /// Create new backup options with default values
    pub fn new(minecraft_path: String, folder_options: Vec<Folders>, destination_path: String, compress: bool) -> Self {
        BackUpOptions {
            default_minecraft_path: minecraft_path,
            folder_options,
            destination_path,
            compress,
            excluded_extensions: Vec::new()
        }
    }

    /// Get all paths based on selected folder options
    pub fn get_all_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        for folder in &self.folder_options {
            let path = match folder {
                Folders::Saves => format!("{}/saves", self.default_minecraft_path),
                Folders::Config => format!("{}/config", self.default_minecraft_path),
                Folders::Screenshots => format!("{}/screenshots", self.default_minecraft_path),
                Folders::Mods => format!("{}/mods", self.default_minecraft_path),
                Folders::Logs => format!("{}/logs", self.default_minecraft_path),
                Folders::Backups => format!("{}/backups", self.default_minecraft_path),
            };
            paths.push(path);
        }

        paths
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }

    pub fn add_excluded_extension(&mut self, extension: String) {
        self.excluded_extensions.push(extension);
    }

    pub fn get_backup_size(&self) ->  u64 {
        let files = self.get_all_files();
        let mut total_size: u64 = 0;

        for file in files.iter() {
            if let Ok(metadata) = std::fs::metadata(file) {
                total_size += metadata.len();
            }
        }

        total_size
    }

    /// Get all files from selected folder options and return all files in the folder as a vector of strings
    pub fn get_all_files(&self) -> Vec<String> {
        let folders: Vec<String> = self.get_all_paths();
        let mut files: Vec<String> = Vec::new();

        for folder in folders.iter() {
            for entry in std::fs::read_dir(folder).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let file_path = path.to_string_lossy().to_string();
                    println!("{}", file_path); // Muestra el archivo encontrado
                    files.push(file_path);
                }
            }
        }

        files
    }
}

impl BackUpData {
    /// Create new backup data
    pub fn new(options: BackUpOptions, size_in_bytes: u64, file_count: u32) -> Self {
        BackUpData {
            options,
            timestamp: SystemTime::now(),
            size_in_bytes,
            file_count
        }
    }

    /// Format backup data as JSON string
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