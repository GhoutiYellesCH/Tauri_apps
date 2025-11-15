// src/io/mod.rs

use tauri::{AppHandle, Manager};
use std::{fs, path::PathBuf};
// Updated imports to include components needed for new vault creation
use crate::crypto::vault_struct::{VaultFile, Header, KdfParams}; 
use crate::crypto::kdf::derive_key;
use crate::crypto::cipher::encrypt_vault; 

const VAULT_FILE_NAME: &str = "vault.json";

// Pass a reference to the AppHandle to access the path resolver
fn get_vault_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut dir = app.path()
        .app_data_dir()
        // FIX: Use map_err to convert tauri::Error to String
        .map_err(|e| e.to_string())?; 
        // FIX: Removed extra semicolon (was '?;?' or '?;;')

    dir.push(VAULT_FILE_NAME);
    Ok(dir)
}

// --- NEW COMMAND 1: Check File Existence ---
/// Checks if the encrypted vault file currently exists on disk.
#[tauri::command]
pub fn check_vault_exists(app: AppHandle) -> Result<bool, String> {
    let path = get_vault_path(&app)?;
    Ok(path.exists())
}

// --- NEW COMMAND 2: Create Initial Vault ---
/// Creates a new, empty, encrypted vault file on disk for a first-time user.
#[tauri::command]
pub fn create_new_vault_file(
    app: AppHandle, 
    master_password: String,
    salt_b64: String, 
    params: KdfParams
) -> Result<(), String> {
    // 1. Derive the session key from the master password and parameters
    let session_key = derive_key(
        &master_password,
        &salt_b64,
        &params,
    )?;

    // 2. Encrypt an empty entry list ("[]")
    // We convert the Zeroizing<[u8; 32]> to Zeroizing<Vec<u8>> for encrypt_vault
    let vault_data = encrypt_vault(session_key.to_vec().into(), "[]".to_string())?;

    // 3. Construct the VaultFile header
    let header = Header {
        kdf_params: params,
        salt: salt_b64,
        cipher_type: "AES-256-GCM".to_string(),
    };

    let vault_file = VaultFile {
        header,
        vault_data,
    };

    // 4. Write the file to disk (using the existing write_vault_file)
    write_vault_file(app, vault_file)?;

    // CRITICAL: The master_password and derived key are zeroized as they go out of scope.

    Ok(())
}

// ... existing write_vault_file and read_vault_file implementations below ...

#[tauri::command]
pub fn write_vault_file(app: AppHandle, vault_file: VaultFile) -> Result<(), String> {
    let path = get_vault_path(&app)?; // Pass a reference

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let json_string = serde_json::to_string_pretty(&vault_file)
        .map_err(|e| format!("Failed to serialize vault: {}", e))?;

    fs::write(&path, json_string)
        .map_err(|e| format!("Failed to write vault file to {}: {}", path.display(), e))?;

    Ok(())
}

#[tauri::command]
pub fn read_vault_file(app: AppHandle) -> Result<VaultFile, String> {
    let path = get_vault_path(&app)?; // Pass a reference

    if !path.exists() {
        return Err(format!("Vault file not found at {}. Try creating a new vault.", path.display()));
    }

    let json_string = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read vault file: {}", e))?;

    serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse vault JSON: {}", e))
}