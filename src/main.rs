mod dir_entry_info;
mod dir_entry_data;
mod dir_entry_processor;
mod processors {
    pub mod sha256_processor;
}
use dir_entry_info::{scan_directory};
use processors::sha256_processor::Sha256Processor;
use crate::dir_entry_processor::DirEntryProcessor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = scan_directory(".\\src").await?;

    let mut processors:Vec<Box<dyn DirEntryProcessor>> = vec![];
    processors.push(Box::new(Sha256Processor::new()));

    for mut file in files {
        println!("Path: {:?}, Is File: {}, Is Dir: {}, String: {}, absPath: {}",
                 file.get_path(), file.is_file(), file.is_dir(),
                 file.get_relative_path_as_string(),
                 file.get_absolute_path_as_string());

        for processors in processors.iter() {
            println!("Processor: {}", processors.name());
            let data = processors.process(&mut file).await?;
            for d in data {
                println!("Data: {:?}", d);
            }
        }

        for data in file.data() {
            println!("Data: {:?}", data);
        }
    }
    Ok(())
}
