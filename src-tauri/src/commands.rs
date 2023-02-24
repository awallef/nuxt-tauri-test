use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;
use zip::read::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod::Deflated;

#[tauri::command]
pub fn echo(message: String) -> Result<(), String> {
    println!("[Rust::echo]: {message}");
    Ok(())
}
#[tauri::command]
pub fn zip_file(path: &str, output_path: &str) -> Result<(), String> {
    println!("[Rust::zip_file]: {path} to {output_path}");

    let input_file_path = Path::new(path);
    let output_file_path = PathBuf::from(output_path);
    let output_file = File::create(output_file_path.clone())
        .map_err(|e| format!("failed to create output file: {}", e))?;
    let mut zip_writer = ZipWriter::new(output_file);
    let options = FileOptions::default().compression_method(Deflated);
    let input_file_name = input_file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("failed to get input file name from path: {}", path))?;
    zip_writer
        .start_file(input_file_name, options)
        .map_err(|e| format!("failed to start zip file entry: {}", e))?;
    let mut input_file =
        File::open(input_file_path).map_err(|e| format!("failed to open input file: {}", e))?;
    io::copy(&mut input_file, &mut zip_writer)
        .map_err(|e| format!("failed to write zip file entry: {}", e))?;
    zip_writer
        .finish()
        .map_err(|e| format!("failed to finish zip file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn unzip_file(path: String) -> Result<(), String> {
    println!("[Rust::unzip_file]: {path}");

    let file_path = Path::new(&path);

    if !file_path.exists() {
        return Err(format!("File not found at path: {}", path));
    }

    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut zip = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
        let out_path = file.sanitized_name();

        if file.is_dir() {
            std::fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut out_file = File::create(&out_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut out_file).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn zip_directory(path: &str, output_path: &str) -> Result<(), String> {
    println!("[Rust::zip_directory]: {path} to {output_path}");

    let output = Command::new("zip")
        .arg("-r")
        .arg(output_path)
        .arg(path)
        .output()
        .map_err(|e| format!("failed to execute zip command: {}", e))?;
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zip command failed: {}", error_message));
    }

    Ok(())
}
