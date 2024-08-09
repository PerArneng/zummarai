mod file_info;

use file_info::{scan_directory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = scan_directory(".\\src").await?;
    for file in files {
        println!("Path: {:?}, Is File: {}, Is Dir: {}, String: {}, absPath: {}",
                 file.get_path(), file.is_file(), file.is_dir(),
                 file.get_relative_path_as_string(),
                 file.get_absolute_path_as_string());
    }
    Ok(())
}
