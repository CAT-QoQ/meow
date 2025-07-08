use serde_json::Value;
use std::fs;
use std::io::Read;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_dir_list(path: &str) -> Result<Value, String> {
    Ok(utils::system::read_path_list_by_sort(path)?)
}

#[tauri::command]
fn read_file_size(path: &str) -> Result<u64, String> {
    Ok(utils::read_file_size(path)?)
}

#[tauri::command]
fn file_seek_read(path: &str, offset: u64, length: usize) -> Result<Vec<u8>, String> {
    Ok(utils::file_seek_read(path, offset, length)?)
}

#[tauri::command]
fn read_file_data(path: &str) -> Result<Value, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let metadata = file
        .metadata()
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if !metadata.is_file() {
        return Err(String::from("Read is not a file"));
    }

    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let modified_time = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|dur| dur.as_secs())
        .unwrap_or(0);

    let mut result = serde_json::Map::new();
    result.insert("is_file".to_string(), Value::from(metadata.is_file()));
    result.insert("size".to_string(), Value::from(metadata.len()));
    result.insert("modified_time".to_string(), Value::from(modified_time));
    result.insert("content".to_string(), Value::from(content));
    Ok(Value::Object(result))
}

#[tauri::command]
fn update_theme_state(is_dark: bool) -> Result<Value, String> {
    #[allow(unused_assignments)]
    #[allow(unused_mut)]
    let mut state = Value::from(is_dark);

    #[cfg(target_os = "android")]
    {
        state = Value::from(utils::android_bridge::toggle_status_bar_theme(!is_dark)?);
    }

    Ok(state)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            read_dir_list,
            read_file_size,
            file_seek_read,
            read_file_data,
            update_theme_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
