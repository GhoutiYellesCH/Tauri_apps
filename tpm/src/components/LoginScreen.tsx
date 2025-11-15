// src/components/LoginScreen.tsx

import React, { useState } from 'react';
import { DEFAULT_KDF_PARAMS } from '../types/backend';
import { invoke } from '@tauri-apps/api/core';

interface LoginScreenProps {
  isLoading: boolean;
  loadVault: (masterPassword: string) => Promise<void>;
}

// Utility function to check if the vault file exists (called before login attempt)
const checkVaultExists = async (): Promise<boolean> => {
    try {
        const pathExists = await invoke<boolean>('check_vault_exists');
        return pathExists;
    } catch {
        return false;
    }
}

const LoginScreen: React.FC<LoginScreenProps> = ({ isLoading, loadVault }) => {
  const [masterPassword, setMasterPassword] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [vaultExists, setVaultExists] = useState(false);

  React.useEffect(() => {
    checkVaultExists().then(setVaultExists);
  }, []);

  const handleLoginOrLoad = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (isCreating && !vaultExists) {
        // --- Create New Vault Flow ---
        try {
            // 1. Generate new KDF salt
            const salt_b64 = await invoke<string>('generate_salt');
            
            // 2. Create the initial vault file on disk (unencrypted empty vault)
            await invoke('create_new_vault_file', { 
                masterPassword, 
                saltB64: salt_b64, 
                params: DEFAULT_KDF_PARAMS 
            });
            
            // After creation, attempt to load it
            await loadVault(masterPassword);
            
        } catch (e) {
            console.error(e);
            alert(`Vault Creation Failed: ${e}`);
        }
        
    } else {
        // --- Load Existing Vault Flow ---
        await loadVault(masterPassword);
    }
  };

  const currentActionText = isCreating && !vaultExists ? 'Create & Login' : 'Login / Load Vault';

  return (
    <div className="max-w-md mx-auto bg-white dark:bg-gray-800 p-8 shadow-2xl rounded-lg">
      <h1 className="text-3xl font-bold mb-6 text-center text-indigo-600 dark:text-indigo-400">VaultGuard</h1>
      <p className="text-center mb-6 text-sm text-gray-500 dark:text-gray-400">
        {vaultExists 
          ? "Enter your Master Password to load the existing vault." 
          : "No vault file found. Enter a password to create a new, empty vault."
        }
      </p>

      <form onSubmit={handleLoginOrLoad}>
        <div className="mb-6">
          <label 
            htmlFor="master-password" 
            className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Master Password
          </label>
          <input
            id="master-password"
            type="password"
            value={masterPassword}
            onChange={(e) => setMasterPassword(e.target.value)}
            required
            className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-indigo-500 focus:border-indigo-500"
            disabled={isLoading}
          />
        </div>

        <button
          type="submit"
          className="w-full flex justify-center py-2 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
          disabled={isLoading || (isCreating && masterPassword.length < 8)}
        >
          {isLoading ? 'Processing...' : currentActionText}
        </button>
      </form>
      
      {!vaultExists && (
        <div className="mt-4 text-center">
            <button 
                className="text-sm text-indigo-600 dark:text-indigo-400 hover:underline"
                onClick={() => setIsCreating(true)}
                disabled={isLoading}
            >
                Start New Vault Creation
            </button>
        </div>
      )}
    </div>
  );
};

export default LoginScreen;