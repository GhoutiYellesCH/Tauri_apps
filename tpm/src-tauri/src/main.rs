// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;
mod io;
mod state;
mod commands;

// Global dependencies for main.rs
use std::sync::Arc;
use tauri::{State, AppHandle}; // Import AppHandle
use base64::{Engine as _, engine::general_purpose::STANDARD}; // Use new Base64 Engine API
use zeroize::Zeroizing;
use serde_json::from_str; // Needed for deserializing entries

// Crate modules
use crate::state::SessionState;
use crate::crypto::vault_struct::{DecryptedVault, PasswordEntry};
use crate::io::{read_vault_file, check_vault_exists, create_new_vault_file};
use crate::crypto::kdf::{derive_key, generate_salt}; 
use crate::crypto::cipher::decrypt_vault;


fn main() {
    let state = Arc::new(SessionState::default());

    tauri::Builder::default()
        // Make the state available to all commands
        .manage(state.clone()) 
        .invoke_handler(tauri::generate_handler![
            // Utility commands
            generate_salt,
            commands::generator::generate_strong_password,
            // ADDED: New Login/Creation commands
            check_vault_exists, 
            create_new_vault_file, 
            
            // Primary flow commands
            load_and_decrypt_vault,
            commands::vault::read_all_entries,
            commands::vault::create_entry,
            commands::vault::update_entry,
            commands::vault::delete_entry,
            commands::vault::save_vault_and_logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn load_and_decrypt_vault(app: AppHandle, master_password: String, state: State<'_, Arc<SessionState>>) -> Result<String, String> {
    // 1. Read the encrypted VaultFile from disk
    let vault_file = read_vault_file(app.clone())?;

    // 2. Derive the key using the password and the header params
    let session_key = derive_key(
        &master_password,
        &vault_file.header.salt,
        &vault_file.header.kdf_params,
    )?;

    // 2b. Convert the raw key bytes to Base64 for decryption command
    let key_b64 = STANDARD.encode(session_key.as_ref());

    // 3. Decrypt the payload
    let decrypted_payload_json = decrypt_vault(key_b64, vault_file.vault_data.clone())?;
    
    // 4. Deserialize into entries
    let entries: Vec<PasswordEntry> = from_str(&decrypted_payload_json)
        .map_err(|e| format!("Failed to parse decrypted JSON into entries: {}", e))?;

    // 5. Build the new session state
    let new_vault_state = DecryptedVault {
        entries: Zeroizing::new(entries),
        // FIX: Convert the Zeroizing<[u8; 32]> to Zeroizing<Vec<u8>> using .to_vec()
        session_key: Zeroizing::new(session_key.to_vec()), 
    };
    
    // 6. Store the state and header
    *state.vault.lock().unwrap() = Some(new_vault_state);
    *state.vault_file_header.lock().unwrap() = Some(vault_file);

    Ok("Vault loaded and stored in secure memory state.".to_string())
}