use aes_gcm::{
    // FIX: Explicitly import all required traits
    aead::{Aead, AeadCore, KeyInit, Nonce, OsRng, AeadInPlace}, 
    Aes256Gcm, Key, Tag // FIX: Import Tag directly
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use zeroize::Zeroizing;
use crate::crypto::vault_struct::VaultData;
use rand::RngCore; // For OsRng.fill_bytes()

// --- Helper Functions ---

fn generate_nonce() -> Nonce<Aes256Gcm> {
    // Explicitly type Nonce::default() to resolve type ambiguity
    let mut nonce = Nonce::<Aes256Gcm>::default();
    OsRng.fill_bytes(&mut nonce);
    nonce
}

// Converts a base64 encoded key string into a Key<Aes256Gcm>
fn key_from_base64(key_b64: String) -> Result<Key<Aes256Gcm>, String> {
    let key_bytes = STANDARD.decode(key_b64)
        .map_err(|e| format!("Base64 decoding failed: {}", e))?;
    
    // FIX (E0599): This call is correct. With KeyInit trait in scope,
    // the compiler will find .map_err() on the returned Result.
    Key::from_slice(&key_bytes)
        .map_err(|_| "Invalid key length for AES-256.".to_string())
}

// --- Main Commands ---

#[tauri::command]
pub fn decrypt_vault(key_b64: String, vault_data: VaultData) -> Result<String, String> {
    let key = key_from_base64(key_b64)?;
    let cipher = Aes256Gcm::new(&key);

    let nonce_bytes = STANDARD.decode(&vault_data.nonce)
        .map_err(|_| "Invalid nonce Base64 encoding.".to_string())?;
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);

    let tag_bytes = STANDARD.decode(&vault_data.tag)
        .map_err(|_| "Invalid tag Base64 encoding.".to_string())?;
    // FIX (E0277/E0599): Use the directly imported 'Tag' type.
    // This is an alias for GenericArray and does not return a Result.
    let tag = Tag::<Aes256Gcm>::from_slice(&tag_bytes);

    let ciphertext = STANDARD.decode(&vault_data.encrypted_payload)
        .map_err(|_| "Invalid encrypted payload Base64 encoding.".to_string())?;

    let mut buffer = ciphertext.to_vec();
    
    // This call is correct because AeadInPlace trait is in scope
    cipher.decrypt_in_place_detached(
        nonce, 
        &[], // No associated data
        &mut buffer, // Pass the mutable buffer
        tag
    ).map_err(|_| "Decryption failed or authentication tag mismatch.".to_string())?;

    let plaintext = String::from_utf8(buffer)
        .map_err(|_| "Decrypted data is not valid UTF-8.".to_string())?;

    Ok(plaintext)
}


pub fn encrypt_vault(session_key_bytes: Zeroizing<Vec<u8>>, entries_json: String) -> Result<VaultData, String> {
    // FIX (E0599): This call is also correct. KeyInit is in scope.
    let key = Key::<Aes256Gcm>::from_slice(&session_key_bytes)
        .map_err(|_| "Session key is the wrong size for AES-256.".to_string())?;
    
    let cipher = Aes256Gcm::new(&key);
    let nonce = generate_nonce();
    let nonce_b64 = STANDARD.encode(nonce.as_slice());

    let ciphertext_and_tag = cipher.encrypt(
        &nonce, 
        entries_json.as_bytes()
    ).map_err(|_| "Encryption failed.".to_string())?;

    let tag_len = 16;
    if ciphertext_and_tag.len() < tag_len {
        return Err("Encryption output is too short.".to_string());
    }
    
    let (ciphertext, tag_bytes) = ciphertext_and_tag.split_at(ciphertext_and_tag.len() - tag_len);
    
    let encrypted_payload = STANDARD.encode(ciphertext);
    let tag = STANDARD.encode(tag_bytes);

    Ok(VaultData {
        nonce: nonce_b64,
        tag,
        encrypted_payload,
    })
}