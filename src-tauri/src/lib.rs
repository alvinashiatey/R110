mod commands;
mod errors;
mod imaging;
mod state;

use commands::*;
use state::create_state;
use tauri::Builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .manage(create_state())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            select_image,
            read_image,
            process_colormap,
            get_processing_status,
            read_processed_images,
            process_selected_image,
            export_channels,
            save_composed_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
