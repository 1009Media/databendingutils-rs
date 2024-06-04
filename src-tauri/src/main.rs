#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tosbr;
mod fromsbr;
mod tohbmp;
mod fromhbmp;

#[tauri::command]
fn convert_to_sbr(input_file: String, output_file: String, header_file: String) -> Result<(), String> {
    tosbr::convert_to_sbr(&input_file, &output_file, &header_file).map_err(|e| e.to_string())
}

#[tauri::command]
fn convert_from_sbr(input_file: String, header_file: String, output_file: String) -> Result<(), String> {
    fromsbr::convert_from_sbr(&input_file, &header_file, &output_file).map_err(|e| e.to_string())
}

#[tauri::command]
fn convert_to_hbmp(input_file: String, output_file: String, header_file: String) -> Result<(), String> {
    tohbmp::convert_to_hbmp(&input_file, &output_file, &header_file).map_err(|e| e.to_string())
}

#[tauri::command]
fn convert_from_hbmp(input_file: String, header_file: String, output_file: String) -> Result<(), String> {
    fromhbmp::convert_from_hbmp(&input_file, &header_file, &output_file).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            convert_to_sbr,
            convert_from_sbr,
            convert_to_hbmp,
            convert_from_hbmp
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
