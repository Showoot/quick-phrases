use crate::error::AppError;
use crate::services::storage::Storage;
use std::fs;
use uuid::Uuid;

/// Copy an image file from source path into the app's images directory.
/// Returns the filename (UUID-based) that can be stored in the Phrase model.
pub fn import_image(storage: &Storage, source_path: &str) -> Result<String, AppError> {
    let source = std::path::Path::new(source_path);
    if !source.exists() {
        return Err(AppError::NotFound(format!(
            "Image file not found: {}",
            source_path
        )));
    }

    let extension = source
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let dest = storage.image_path(&filename);

    fs::copy(source, &dest)?;
    Ok(filename)
}

/// Delete an image file from the images directory
pub fn delete_image(storage: &Storage, filename: &str) -> Result<(), AppError> {
    let path = storage.image_path(filename);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Read an image file and return as base64-encoded string
pub fn read_image_base64(storage: &Storage, filename: &str) -> Result<String, AppError> {
    let path = storage.image_path(filename);
    if !path.exists() {
        return Err(AppError::NotFound(format!("Image not found: {}", filename)));
    }
    let bytes = fs::read(&path)?;
    Ok(base64_encode(&bytes))
}

fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let chunks = bytes.chunks(3);
    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}
