use crate::errors::Error;
use crate::imaging::processes::{apply_colormap, process_image, process_image_background};
use crate::state::{AppState, ProcessSettings, ProcessingStatus};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use log;
use std::fs;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, serde::Serialize)]
pub struct AppResponse {
    processed_images: Option<Vec<crate::imaging::processes::ProcessResult>>,
    image_path: String,
    image_type: String,
    image_name: String,
    processing_status: ProcessingStatus,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessingCompletePayload {
    processed_images: Option<Vec<crate::imaging::processes::ProcessResult>>,
    status: ProcessingStatus,
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
        let mut state_lock = state.lock().unwrap();

        let image_type = selected_path
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_string())
            .unwrap_or_default();

        let image_name = selected_path
            .rsplit_once('/')
            .map(|(_, name)| name.to_string())
            .unwrap_or_default();

        // Store in state
        state_lock.image_path = Some(selected_path.clone());
        state_lock.current_image = Some(selected_path.clone());
        state_lock.image_type = Some(image_type.clone());
        state_lock.image_name = Some(image_name.clone());
        state_lock.processing_status = ProcessingStatus::Processing;

        // Clone what we need for the background task
        let image_path = selected_path.clone();
        let app_handle = app.clone();

        // Create a background task to process the image
        tauri::async_runtime::spawn(async move {
            // Process the image in the background
            match process_image_background(&image_path) {
                Ok(result) => {
                    let app_state = app_handle.state::<AppState>();
                    let mut state = app_state.lock().unwrap();
                    // Store in a separate field, NOT processed_images
                    state.preprocessed_channels = Some(result);
                    // We don't update processing_status to Completed yet,
                    // nor do we emit the event that updates the UI.
                    log::info!("Background channel separation completed and cached");
                }
                Err(e) => {
                    log::error!("Background processing failed: {}", e);
                }
            };
        });

        // Return immediate response
        Ok(AppResponse {
            processed_images: None,
            image_path: selected_path,
            image_type,
            image_name,
            processing_status: ProcessingStatus::Processing,
        })
    } else {
        Err(Error::NoImageSelected)
    }
}

#[tauri::command]
pub fn get_processing_status(state: State<'_, AppState>) -> ProcessingStatus {
    state.lock().unwrap().processing_status.clone()
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

#[tauri::command]
pub async fn read_processed_images(state: State<'_, AppState>) -> Result<Vec<[String; 2]>, Error> {
    let state = state.lock().unwrap();
    if let Some(ref processed_images) = state.processed_images {
        let mut images = vec![];
        for img in processed_images {
            let image_bytes = fs::read(&img.image_path)?;
            let base64_string = base64_engine.encode(&image_bytes);
            images.push([base64_string, img.channel.clone()]);
        }
        Ok(images)
    } else {
        Err(Error::NoImageSelected)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn process_colormap(
    _: State<'_, AppState>,
    image_path: String,
    hex_color: String,
) -> Result<String, Error> {
    let colormap = apply_colormap(&image_path, &hex_color)?;
    Ok(colormap)
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

        match process_image(&path, &state, state.preprocessed_channels.as_ref()) {
            Ok(processed_result) => {
                state.processed_images = Some(processed_result.clone());
                Ok(AppResponse {
                    processed_images: Some(processed_result),
                    image_path: path,
                    image_type: state.image_type.clone().unwrap_or_default(),
                    image_name: state.image_name.clone().unwrap_or_default(),
                    processing_status: state.processing_status.clone(),
                })
            }
            Err(e) => Err(e),
        }
    } else {
        Err(Error::NoImageSelected)
    }
}
