mod dir_entry_info;
mod dir_entry_data;
mod dir_entry_processor;
mod processors {
    pub mod sha256_processor;
}

use env_logger::{Builder, Env};
use log::info;
use dir_entry_info::{scan_directory};
use processors::sha256_processor::Sha256Processor;
use crate::dir_entry_processor::DirEntryProcessor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting");

    let files = scan_directory(".\\src").await?;

    let mut processors:Vec<Box<dyn DirEntryProcessor>> = vec![];
    processors.push(Box::new(Sha256Processor::new()));

    for mut file in files {
        info!("path: {:?}, is_file: {}", file.get_path(), file.is_file());

        for processors in processors.iter() {
            info!("running processor: {} on {}", processors.name(), file.get_path().display());
            let data = processors.process(&mut file).await?;
            for d in data {
                file.add_data(d);
            }
        }

        for data in file.data() {
            info!("data: name({}) mime({}) <- {:?} ", data.name(), data.mime_type(), file.get_path());
        }
    }
    Ok(())
}
