use crate::errors::Error;
use crate::imaging::processes::process_image;
use crate::state::AppState;
use crate::state::ProcessSettings;
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use log;
use std::fs;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, serde::Serialize)]
pub struct AppResponse {
    image_path: String,
    image_type: String,
    image_name: String,
}

#[tauri::command]
pub async fn select_image(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<AppResponse, Error> {
    if let Some(selected_path) = app
        .dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .blocking_pick_file()
        .map(|p| p.to_string())
    {
        let mut state = state.lock().unwrap();

        let image_type = selected_path
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_string())
            .unwrap_or_default();

        let image_name = selected_path
            .rsplit_once('/')
            .map(|(_, name)| name.to_string())
            .unwrap_or_default();

        // Store in state
        state.image_path = Some(selected_path.clone());
        state.current_image = Some(selected_path.clone());
        state.image_type = Some(image_type.clone());
        state.image_name = Some(image_name.clone());

        // Return response
        Ok(AppResponse {
            image_path: selected_path,
            image_type,
            image_name,
        })
    } else {
        Err(Error::NoImageSelected)
    }
}

#[tauri::command]
pub fn read_image(state: State<'_, AppState>) -> Result<String, Error> {
    let state = state.lock().unwrap();
    if let Some(ref path) = state.current_image {
        let image_bytes = fs::read(path)?;
        let base64_string = base64_engine.encode(&image_bytes);
        Ok(base64_string)
    } else {
        Err(Error::NoImageSelected)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn process_selected_image(
    state: State<'_, AppState>,
    process_data: ProcessSettings,
) -> Result<AppResponse, Error> {
    let mut state = state.lock().unwrap();
    if let Some(path) = state.image_path.clone() {
        // Update state with new process settings
        state.process_settings = Some(process_data);
        log::info!("Processing image: {:?}", state.process_settings);

        match process_image(&path, &state) {
            Ok(new_image_path) => {
                state.current_image = Some(new_image_path.clone());
                let image_type = state.image_type.clone().unwrap_or_default();
                let image_name = state.image_name.clone().unwrap_or_default();

                Ok(AppResponse {
                    image_path: new_image_path,
                    image_type,
                    image_name,
                })
            }
            Err(e) => Err(e),
        }
    } else {
        Err(Error::NoImageSelected)
    }
}
