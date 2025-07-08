use std::fs;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub mod system;
#[cfg(target_os = "android")]
pub mod android_bridge;

pub fn read_file_size(file_path: &str) -> Result<u64, String> {
    let meta = fs::metadata(file_path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    Ok(meta.len()) // 返回文件大小
}

pub fn file_seek_read(file_path: &str, offset: u64, length: usize) -> Result<Vec<u8>, String> {
    let file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader = BufReader::new(file);
    reader
        .seek(SeekFrom::Start(offset))
        .map_err(|e| format!("Failed to seek offset file: {}", e))?;
    let mut buffer = vec![0u8; length];
    // 移动到指定偏移量
    let bytes_read = reader
        .read(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}
