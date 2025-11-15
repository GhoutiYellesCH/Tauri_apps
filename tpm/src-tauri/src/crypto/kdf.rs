use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    // FIX: Explicitly import the PasswordHasher trait
    Argon2, Params as Argon2Params, PasswordHasher
};
// FIX: RngCore is needed for .fill_bytes()
use rand::RngCore; 
use zeroize::Zeroizing;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::crypto::vault_struct::KdfParams;

const KEY_LEN: usize = 32;

pub fn derive_key(master_password: &str, salt: &str, params: &KdfParams) -> Result<Zeroizing<[u8; KEY_LEN]>, String> {
    let salt = SaltString::from_b64(salt)
        .map_err(|e| format!("Invalid salt format: {}", e))?;

    let argon_params = Argon2Params::new(
        params.memory_cost, 
        params.time_cost, 
        params.parallelism,
        Some(KEY_LEN),
    ).map_err(|e| format!("Invalid KDF parameters: {}", e))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon_params,
    );

    let mut key_bytes = Zeroizing::new([0u8; KEY_LEN]);

    // FIX: This call now works because the PasswordHasher trait is in scope
    argon2.hash_password_into(
        master_password.as_bytes(),
        salt.as_ref().as_bytes(),
        &mut *key_bytes
    ).map_err(|e| format!("Key derivation failed: {}", e))?;

    Ok(key_bytes)
}

#[tauri::command]
pub fn generate_salt() -> Result<String, String> {
    let mut salt_bytes = [0u8; 16];
    // FIX: This call now works because the RngCore trait is in scope
    OsRng.fill_bytes(&mut salt_bytes);
    Ok(STANDARD.encode(salt_bytes))
}