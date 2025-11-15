use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, Zeroizing};

// KDF Parameters struct for IPC command and header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdfParams {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        KdfParams {
            memory_cost: 65536, // 64 MiB
            time_cost: 4,
            parallelism: 1,
        }
    }
}

// The Core Password Entry (Decrypted, held in volatile memory)
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
#[zeroize(drop)]
pub struct PasswordEntry {
    pub id: String, 
    pub name: String,
    pub username: String,
    // CRITICAL: This is plaintext only when in memory.
    pub password: String, 
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

// The Encrypted Payload Structure (Stored in VaultFile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultData {
    pub nonce: String,           // Base64 encoded Nonce/IV
    pub tag: String,             // Base64 encoded GCM Authentication Tag
    pub encrypted_payload: String, // Base64 encoded ciphertext
}

// The Unencrypted Header (Stored in VaultFile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub kdf_params: KdfParams,
    pub salt: String, // Base64 encoded KDF salt
    pub cipher_type: String, // E.g., "AES-256-GCM"
}

// The Complete Vault File Structure (JSON written to disk)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFile {
    pub header: Header,
    pub vault_data: VaultData,
}

// In-Memory Decrypted Vault State (for Tauri's State Manager)
#[derive(Debug, Clone)]
pub struct DecryptedVault {
    pub session_key: Zeroizing<Vec<u8>>,
    pub entries: Zeroizing<Vec<PasswordEntry>>, 
}

impl Zeroize for DecryptedVault {
    fn zeroize(&mut self) {
        // Explicitly zeroize sensitive data on drop/logout
        self.session_key.zeroize();
        self.entries.zeroize();
    }
}