use tauri::{State, AppHandle};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::SessionState;
use crate::crypto::vault_struct::{PasswordEntry, VaultFile};
use crate::crypto::cipher::encrypt_vault;
use crate::io::write_vault_file;
use zeroize::Zeroize; 
use serde::Deserialize;

// --- Helper Types ---

#[derive(Debug, Clone, Deserialize)]
pub struct OmitIdPasswordEntry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}


// --- Helper Functions ---

// Helper to access the mutable vault state and check for login
fn get_vault_state<'a>(state: &'a State<'a, Arc<SessionState>>) -> Result<std::sync::MutexGuard<'a, Option<crate::crypto::vault_struct::DecryptedVault>>, String> {
    let vault_guard = state.vault.lock().map_err(|_| "Internal error: Failed to lock vault state.".to_string())?;
    if vault_guard.is_none() {
        return Err("Not logged in. Vault is not loaded.".to_string());
    }
    Ok(vault_guard)
}

// Helper to access the mutable header state
fn get_header_state<'a>(state: &'a State<'a, Arc<SessionState>>) -> Result<std::sync::MutexGuard<'a, Option<VaultFile>>, String> {
    state.vault_file_header.lock().map_err(|_| "Internal error: Failed to lock vault header.".to_string())
}


// --- Tauri Commands (CRUD) ---

/// Command 1: Read all entries from the secure, in-memory vault.
#[tauri::command]
pub fn read_all_entries(state: State<'_, Arc<SessionState>>) -> Result<Vec<PasswordEntry>, String> {
    let vault_guard = get_vault_state(&state)?;
    // Clone the entries to return them, leaving the Zeroizing container locked
    Ok(vault_guard.as_ref().unwrap().entries.to_vec())
}


/// Command 2: Create a new password entry and add it to the in-memory vault.
#[tauri::command]
pub fn create_entry(state: State<'_, Arc<SessionState>>, new_entry: OmitIdPasswordEntry) -> Result<PasswordEntry, String> {
    let mut vault_guard = get_vault_state(&state)?;
    let vault = vault_guard.as_mut().unwrap();

    let new_id = Uuid::new_v4().to_string();

    let entry = PasswordEntry {
        id: new_id,
        name: new_entry.name,
        username: new_entry.username,
        password: new_entry.password,
        url: new_entry.url,
        notes: new_entry.notes,
        tags: new_entry.tags,
    };

    // Push the new entry into the Zeroizing Vec<PasswordEntry>
    vault.entries.push(entry.clone());

    Ok(entry)
}

/// Command 3: Update an existing password entry.
#[tauri::command]
pub fn update_entry(state: State<'_, Arc<SessionState>>, updated_entry: PasswordEntry) -> Result<String, String> {
    let mut vault_guard = get_vault_state(&state)?;
    let vault = vault_guard.as_mut().unwrap();

    let entries = &mut vault.entries;
    
    // Find the index of the entry to update
    if let Some(index) = entries.iter().position(|e| e.id == updated_entry.id) {
        // Replace the existing entry with the updated one
        entries[index] = updated_entry;
        Ok("Entry updated successfully.".to_string())
    } else {
        Err(format!("Entry with ID {} not found.", updated_entry.id))
    }
}

/// Command 4: Delete an entry by ID.
#[tauri::command]
pub fn delete_entry(state: State<'_, Arc<SessionState>>, id: String) -> Result<String, String> {
    let mut vault_guard = get_vault_state(&state)?;
    let vault = vault_guard.as_mut().unwrap();

    let entries = &mut vault.entries;
    
    let original_len = entries.len();
    
    // Retain only entries whose ID does NOT match the ID to be deleted
    entries.retain(|e| e.id != id);

    if entries.len() < original_len {
        // Entries were successfully deleted
        // NOTE: The Zeroizing ensures the memory used by the deleted entry is zeroed
        // when the Vec is resized, but PasswordEntry is also marked for Zeroize on drop.
        
        // This *re-sets* the entries in the Zeroizing container, triggering zeroization.
        let updated_entries = entries.clone(); 
        *entries = updated_entries; 

        Ok("Entry deleted successfully.".to_string())
    } else {
        Err(format!("Entry with ID {} not found.", id))
    }
}


/// Command 5: Save the vault to disk (Encryption happens here) and log out.
#[tauri::command]
pub fn save_vault_and_logout(app: AppHandle, state: State<'_, Arc<SessionState>>) -> Result<(), String> {
    // 1. Get current states (and ensure we are logged in)
    let vault_guard = get_vault_state(&state)?;
    let header_guard = get_header_state(&state)?;

    let vault = vault_guard.as_ref().unwrap();
    let header = header_guard.as_ref().ok_or_else(|| "Vault header is missing.".to_string())?;

    // 2. Serialize the in-memory entries into a JSON string
    let entries_json = serde_json::to_string(&*vault.entries)
        .map_err(|e| format!("Failed to serialize entries: {}", e))?;

    // 3. Encrypt the JSON payload
    let vault_data = encrypt_vault(vault.session_key.clone(), entries_json)?;

    // 4. Construct the final VaultFile
    let new_vault_file = VaultFile {
        header: header.header.clone(),
        vault_data,
    };

    // 5. Write the file to disk
    write_vault_file(app, new_vault_file)?;

    let session_state = state.inner();

    if let Ok(mut vault_guard) = session_state.vault.lock() {
        if let Some(mut vault) = vault_guard.take() {
            // DecryptedVault implements Zeroize, and .take() hands over ownership to 'vault'
            // which will then be dropped, running the Zeroize implementation
            vault.zeroize();
        }
    }

    // Clear the VaultFileHeader from the Mutex
    if let Ok(mut header_guard) = session_state.vault_file_header.lock() {
        *header_guard = None;
    }

    Ok(())
}