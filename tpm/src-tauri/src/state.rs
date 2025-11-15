use std::sync::Mutex;
use crate::crypto::vault_struct::{DecryptedVault, VaultFile};
use zeroize::Zeroize;

// The struct that holds the state of the active session
#[derive(Default)]
pub struct SessionState {
    // Contains the decrypted entries and the session key
    pub vault: Mutex<Option<DecryptedVault>>,
    // Contains the unencrypted header data needed for saving
    pub vault_file_header: Mutex<Option<VaultFile>>,
}

// Logic for safely removing sensitive data from memory
impl Zeroize for SessionState {
    fn zeroize(&mut self) {
        if let Ok(mut vault_guard) = self.vault.lock() {
            if let Some(mut vault) = vault_guard.take() {
                vault.zeroize();
            }
        }
        if let Ok(mut header_guard) = self.vault_file_header.lock() {
            *header_guard = None;
        }
    }
}