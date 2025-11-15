use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Params, PasswordHasher,
};
use zeroize::Zeroizing;
use base64;
use crate::crypto::vault_struct::KdfParams;

// Exposed command to generate a new secure salt (Base64 encoded)
#[tauri::command]
pub fn generate_salt() -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(base64::encode(salt.as_bytes()))
}

// CORE KDF IMPLEMENTATION: Derives a 32-byte (256-bit) encryption key
#[tauri::command]
pub fn derive_key(
    master_password: String,
    salt_b64: String,
    params: KdfParams,
) -> Result<String, String> {
    let mut password_clear = Zeroizing::new(master_password);

    let salt_bytes = base64::decode(&salt_b64)
        .map_err(|e| format!("Salt decoding failed: {}", e))?;
    
    if salt_bytes.len() != 16 {
        return Err("Invalid salt length.".into());
    }

    let argon2_params = Params::new(
        params.memory_cost,
        params.time_cost,
        params.parallelism,
        Some(32), // KEY LENGTH: 32 bytes (256 bits)
    )
    .map_err(|e| format!("KDF parameter error: {}", e))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2_params,
    );
    
    let mut key_bytes = [0u8; 32];
    argon2.hash_password_into(
        password_clear.as_bytes(),
        &salt_bytes,
        &mut key_bytes
    )
    .map_err(|e| format!("Argon2id derivation failed: {}", e))?;

    password_clear.zeroize();

    let key_b64 = base64::encode(&key_bytes);
    
    // CRITICAL: Zeroize the derived key bytes immediately after encoding
    key_bytes.zeroize(); 

    Ok(key_b64)
}