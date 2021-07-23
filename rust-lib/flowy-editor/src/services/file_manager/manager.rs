use crate::services::file_manager::*;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{PoisonError, RwLock, RwLockReadGuard},
};

pub(crate) struct FileManagerConfig {
    doc_dir: String,
}

impl FileManagerConfig {
    pub fn new(root: &str) -> Self {
        let doc_dir = format!("{}/doc", root);
        Self { doc_dir }
    }
}

pub struct FileManager {
    config: FileManagerConfig,
    open_files: HashMap<PathBuf, FileId>,
    file_info: HashMap<FileId, FileInfo>,
}

impl FileManager {
    pub(crate) fn new(config: FileManagerConfig) -> Self {
        // let _ = create_dir_if_not_exist(&config.doc_dir)?;
        Self {
            config,
            open_files: HashMap::new(),
            file_info: HashMap::new(),
        }
    }

    pub(crate) fn open<T>(&mut self, path: &Path, id: T) -> Result<String, FileError>
    where
        T: Into<FileId>,
    {
        if !path.exists() {
            return Ok("".to_string());
        }
        let file_id = id.into();
        let (s, info) = try_load_file(path)?;
        self.open_files.insert(path.to_owned(), file_id.clone());
        self.file_info.insert(file_id, info);

        Ok(s)
    }

    pub(crate) fn save<T>(&mut self, path: &Path, text: &String, id: T) -> Result<(), FileError>
    where
        T: Into<FileId>,
    {
        let file_id = id.into();
        let is_existing = self.file_info.contains_key(&file_id);
        if is_existing {
            self.save_existing(path, text, &file_id)
        } else {
            self.save_new(path, text, &file_id)
        }
    }

    pub(crate) fn close<T>(&mut self, id: T)
    where
        T: Into<FileId>,
    {
        if let Some(file_info) = self.file_info.remove(&id.into()) {
            self.open_files.remove(&file_info.path);
        }
    }

    pub(crate) fn make_file_path(&self, id: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.config.doc_dir, id))
    }

    pub(crate) fn get_info(&self, id: &FileId) -> Option<&FileInfo> { self.file_info.get(id) }

    pub(crate) fn get_file_id(&self, path: &Path) -> Option<FileId> {
        self.open_files.get(path).cloned()
    }

    pub fn check_file(&mut self, path: &Path, id: &FileId) -> bool {
        if let Some(info) = self.file_info.get_mut(&id) {
            let modified_time = get_modified_time(path);
            if modified_time != info.modified_time {
                info.has_changed = true
            }
            return info.has_changed;
        }
        false
    }

    fn save_new(&mut self, path: &Path, text: &String, id: &FileId) -> Result<(), FileError> {
        try_save(path, text, CharacterEncoding::Utf8, self.get_info(id))
            .map_err(|e| FileError::Io(e, path.to_owned()))?;
        let info = FileInfo {
            encoding: CharacterEncoding::Utf8,
            path: path.to_owned(),
            modified_time: get_modified_time(path),
            has_changed: false,
        };
        self.open_files.insert(path.to_owned(), id.clone());
        self.file_info.insert(id.clone(), info);
        Ok(())
    }

    fn save_existing(&mut self, path: &Path, text: &String, id: &FileId) -> Result<(), FileError> {
        let prev_path = self.file_info[id].path.clone();
        if prev_path != path {
            self.save_new(path, text, id)?;
            self.open_files.remove(&prev_path);
        } else if self.file_info[&id].has_changed {
            return Err(FileError::HasChanged(path.to_owned()));
        } else {
            let encoding = self.file_info[&id].encoding;
            try_save(path, text, encoding, self.get_info(id))
                .map_err(|e| FileError::Io(e, path.to_owned()))?;
            self.file_info.get_mut(&id).unwrap().modified_time = get_modified_time(path);
        }
        Ok(())
    }
}
