use std::{
    fs::{write, read_dir, File, metadata, remove_file},
    time::SystemTime,
    io::{Result, copy},
    path::Path,
};

#[derive(Debug, Clone)]
pub enum Folders {
    Saves,
    Config,
    Screenshots,
    Mods,
    Logs,
    Backups
}

#[derive(Clone, Debug)]
pub struct BackUpOptions {
    pub default_minecraft_path: String,
    pub folder_options: Vec<Folders>,
    pub destination_path: String,
    pub compress: bool,
    pub excluded_extensions: Vec<String>
}

pub struct BackUpData {
    pub options: BackUpOptions,
    pub timestamp: SystemTime,
    pub size_in_bytes: u64,
    pub file_count: u32,
    pub json_size_in_bytes: u64
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

    /// Get total size of all files in selected folders
    pub fn get_backup_size(&self) ->  u64 {
        let files = self.get_all_files();
        let mut total_size: u64 = 0;

        for file in files.iter() {
            if let Ok(metadata) = metadata(file) {
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
            if let Ok(entries) = read_dir(folder) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if let Some(ext_str) = ext.to_str() {
                                    if self.excluded_extensions.contains(&ext_str.to_string()) {
                                        continue;
                                    }
                                }
                            }
                            let file_path = path.to_string_lossy().to_string();
                            files.push(file_path);
                        }
                    }
                }
            }
        }

        files
    }
}

impl BackUpData {
    /// Create new backup data
    pub fn new(options: BackUpOptions, size_in_bytes: u64) -> Self {
        BackUpData {
            options,
            timestamp: SystemTime::now(),
            size_in_bytes,
            file_count: 0,
            json_size_in_bytes: 0
        }
    }

    /// Create a JSON file with the backup data
    pub fn create_json_file(&mut self) -> Result<()> {
        let json_data = self.format_json();
        let output_path = format!("{}/backup_data.json", self.options.destination_path);
        println!("Creating backup data file: {:?}", &output_path);
        self.json_size_in_bytes = json_data.len() as u64;
        write(output_path, json_data)
    }

    /// Format backup data as JSON string
    pub fn format_json(&mut self) -> String {
        self.count_files();

        let timestamp = match self.timestamp.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur.as_secs(),
            Err(_) => 0,
        };

        format!(
            r#"{{
                "timestamp": {},
                "size_in_bytes": {},
                "file_count": {},
                "options": {{
                    "folder_options": {:?},
                    "destination_path": "{}",
                    "compress": {},
                    "excluded_extensions": {:?}
                }}
            }}"#,
            timestamp,
            self.size_in_bytes,
            self.file_count,
            self.options.folder_options,
            self.options.destination_path,
            self.options.compress,
            self.options.excluded_extensions
        )
    }

    /// Count total number of files in the backup
    pub fn count_files(&mut self) {
        let files = self.options.get_all_files();
        self.file_count = files.len() as u32;
    }
}

impl BackupManager {
    /// Create a zip backup of the selected folders
    pub fn zip_backup(options: &BackUpOptions) {
        let files = options.get_all_files();
        let zip_path = if options.compress {
            format!("{}/backup.zip", options.destination_path)
        } else {
            format!("{}/backup", options.destination_path)
        };

        let file = File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);

        let options_var: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        // Add all selected files
        for file_path in files.iter() {
            let path = Path::new(file_path);
            let rel_path = path.strip_prefix(&options.default_minecraft_path)
                .unwrap_or(path);
            let rel_path_str = rel_path.to_string_lossy();
            zip.start_file(rel_path_str, options_var).unwrap();
            let mut f = File::open(path).unwrap();
            copy(&mut f, &mut zip).unwrap();
        }

        // Add the backup_data.json to the zip
        let json_path = format!("{}/backup_data.json", options.destination_path);
        if Path::new(&json_path).exists() {
            zip.start_file("backup_data.json", options_var).unwrap();
            let mut f = File::open(&json_path).unwrap();
            copy(&mut f, &mut zip).unwrap();
            // Borrar el json después de añadirlo al zip
            remove_file(&json_path).unwrap();
        }

        zip.finish().unwrap();
    }
}