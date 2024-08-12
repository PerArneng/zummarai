use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt};
use crate::dir_entry_data::DirEntryData;
use crate::dir_entry_info::DirEntryInfo;
use crate::dir_entry_processor::DirEntryProcessor;
use sha2::{Sha256, Digest};

pub(crate) struct Sha256Processor;

impl Sha256Processor {

    pub(crate) fn new() -> Self {
        Sha256Processor
    }

}

// remove this annotation when async-trait is no longer needed:
//   https://docs.rs/async-trait/latest/async_trait/
//   Roadmap: https://github.com/orgs/rust-lang/projects/28/views/2
#[async_trait::async_trait]
impl DirEntryProcessor for Sha256Processor {

    fn name(&self) -> &str {
        "sha256"
    }

    async fn process(&self, dir_entry_info: &mut DirEntryInfo)
                     -> Result<Vec<DirEntryData>, std::io::Error> {

        let mut data = Vec::new();

        if !dir_entry_info.is_file() {
            // make a hash of all the hashes of subfiles and subdirs
        } else {
            data.push(DirEntryData::new_with_string_data(
                mime::TEXT_PLAIN,
                "sha256".to_string(),
                sha256_of_file(dir_entry_info.get_absolute_path()).await?
            ));
        }

        Ok(data)
    }

}

async fn sha256_of_file<P>(path: P) -> io::Result<String>
    where P: AsRef<Path> {

    //println!("Calculating SHA256 for file: {:?}", path.as_ref());

    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}