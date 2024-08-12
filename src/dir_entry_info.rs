use std::path::{PathBuf, Path};
use tokio::fs::{self, ReadDir};
use std::collections::VecDeque;
use crate::dir_entry_data::DirEntryData;

pub(crate) struct DirEntryInfo {
    path: PathBuf,
    absolute_path: PathBuf,
    data: Vec<DirEntryData>
}

impl DirEntryInfo {

    async fn new<P>(path: P) -> std::io::Result<Self>
        where P: AsRef<Path> {
        let path = path.as_ref();
        let abs_path = get_absolute_path(path).await?;
        Ok(DirEntryInfo {
            path: path.to_path_buf(),
            absolute_path: abs_path,
            data: Vec::new()
        })
    }

    pub(crate) fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub(crate) fn get_relative_path_as_string(&self) -> String {
        self.path.display().to_string()
    }

    pub(crate) fn get_absolute_path(&self) -> &PathBuf {
        &self.absolute_path
    }

    pub(crate) fn get_absolute_path_as_string(&self) -> String {
        self.absolute_path.display().to_string()
    }

    pub(crate) fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub(crate) fn add_data(&mut self, data: DirEntryData) {
        self.data.push(data);
    }

    pub(crate) fn data(&self) -> &Vec<DirEntryData> {
        &self.data
    }
}

pub(crate) async fn scan_directory<P: AsRef<Path>>(root: P) -> Result<Vec<DirEntryInfo>, std::io::Error> {
    let root = root.as_ref().to_path_buf();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(dir) = queue.pop_front() {

        result.push(DirEntryInfo::new(&dir).await?);

        let mut entries: ReadDir = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_info = DirEntryInfo::new(&path).await?;
            result.push(file_info);

            if path.is_dir() {
                queue.push_back(path);
            }
        }
    }

    Ok(result)
}

async fn get_absolute_path<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    fs::canonicalize(path).await
}