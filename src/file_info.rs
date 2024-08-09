use std::path::{PathBuf, Path};
use tokio::fs::{self, ReadDir};
use std::collections::VecDeque;

pub(crate) struct FileInfo {
    path: PathBuf,
    absolute_path: PathBuf,
}

impl FileInfo {
    async fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let abs_path = get_absolute_path(path).await?;
        Ok(FileInfo {
            path: path.to_path_buf(),
            absolute_path: abs_path,
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

}

pub(crate) async fn scan_directory<P: AsRef<Path>>(root: P) -> Result<Vec<FileInfo>, std::io::Error> {
    let root = root.as_ref().to_path_buf();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(dir) = queue.pop_front() {
        let mut entries: ReadDir = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_info = FileInfo::new(&path).await?;
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