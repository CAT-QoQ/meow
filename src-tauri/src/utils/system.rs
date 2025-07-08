use rayon::prelude::*;
use serde_json::{json, Value};
use std::fs;

#[cfg(target_os = "windows")]
pub fn read_local_drive_list() -> Vec<String> {
    use windows::Win32::Storage::FileSystem::GetLogicalDriveStringsW;
    unsafe {
        let mut buffer = [0u16; 128];
        let len = GetLogicalDriveStringsW(Some(&mut buffer));
        let buffer = &buffer[..len as usize];
        let result = buffer
            .split(|&c| c == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf16_lossy(s))
            .collect::<Vec<_>>();
        result
    }
}

#[allow(dead_code)]
pub fn read_path_list(path: &str) -> Result<Value, String> {
    #[allow(unused_mut)]
    let mut path = String::from(path);

    #[cfg(target_os = "windows")]
    if path.is_empty() {
        let disk_list = read_local_drive_list()
            .iter()
            .map(|disk| {
                let mut drive = serde_json::Map::<String, Value>::new();
                drive.insert("is_dir".to_string(), Value::Bool(true));
                drive.insert("name".to_string(), Value::String(disk.to_string()));
                Value::Object(drive)
            })
            .collect::<Vec<_>>();
        return Ok(json!(disk_list));
    }

    #[cfg(target_os = "android")]
    if path.is_empty() {
        let mut drive = serde_json::Map::<String, Value>::new();
        drive.insert("is_dir".to_string(), Value::Bool(true));
        drive.insert("name".to_string(), Value::from("/sdcard"));
        let disk_list = Ok(json!(vec![Value::Object(drive)]));
        return disk_list;
    }

    // 尝试读取目录，若失败返回自定义错误信息
    let entries = fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;
    Ok(json!(entries
        .filter_map(|entry| entry.ok()) // 过滤出成功的条目
        .map(|disk| {
            let mut drive = serde_json::Map::<String, Value>::new();
            drive.insert("is_dir".to_string(), Value::Bool(disk.path().is_dir()));
            drive.insert(
                "name".to_string(),
                Value::String(disk.file_name().to_string_lossy().to_string()),
            );
            Value::Object(drive)
        })
        .collect::<Vec<_>>()))
}

pub fn read_path_list_by_sort(path: &str) -> Result<Value, String> {
    #[allow(unused_mut)]
    let mut path = String::from(path);

    #[cfg(target_os = "windows")]
    if path.is_empty() {
        let disk_list = read_local_drive_list()
            .iter()
            .map(|disk| {
                let mut drive = serde_json::Map::<String, Value>::new();
                drive.insert("is_dir".to_string(), Value::Bool(true));
                drive.insert("name".to_string(), Value::String(disk.to_string()));
                Value::Object(drive)
            })
            .collect::<Vec<_>>();
        return Ok(json!(disk_list));
    }

    #[cfg(target_os = "android")]
    if path.is_empty() {
        let mut drive = serde_json::Map::<String, Value>::new();
        drive.insert("is_dir".to_string(), Value::Bool(true));
        drive.insert("name".to_string(), Value::from("/sdcard"));
        let disk_list = Ok(json!(vec![Value::Object(drive)]));
        return disk_list;
    }

    // 尝试读取目录，若失败返回自定义错误信息
    let entries = fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;
    let mut metadata_list = entries
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .filter_map(|entry| {
            let mut drive = serde_json::Map::<String, Value>::new();
            let metadata = entry.metadata().ok()?; // 获取元数据
            let modified_time = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|dur| dur.as_secs())
                .unwrap_or(0); // 获取修改时间
            let file_name = entry.file_name().to_string_lossy().to_string();
            drive.insert("is_dir".to_string(), Value::from(metadata.is_dir()));
            drive.insert("modified_time".to_string(), Value::from(modified_time));
            drive.insert("name".to_string(), Value::from(file_name));
            Some(Value::from(drive))
        })
        .collect::<Vec<_>>();

    metadata_list.sort_by_key(|entry| {
        entry
            .get("modified_time")
            .and_then(|v| v.as_u64()) // 获取排序字段
            .unwrap_or(0) // 如果不存在，默认值为 0
    });
    metadata_list.reverse(); // 从新到旧排序

    Ok(json!(metadata_list))
}
