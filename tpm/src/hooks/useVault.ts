// src/hooks/useVault.ts

import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { PasswordEntry } from '../types/backend';

// Define the shape of the custom hook's return value
interface VaultContext {
  entries: PasswordEntry[];
  isLoggedIn: boolean;
  isLoading: boolean;
  error: string | null;
  loadVault: (masterPassword: string) => Promise<void>;
  saveAndLogout: () => Promise<void>;
  createEntry: (entry: Omit<PasswordEntry, 'id'>) => Promise<void>;
  updateEntry: (entry: PasswordEntry) => Promise<void>;
  deleteEntry: (id: string) => Promise<void>;
  readEntries: () => Promise<void>;
}

export const useVault = (): VaultContext => {
  const [entries, setEntries] = useState<PasswordEntry[]>([]);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // --- Utility Functions ---

  const readEntries = useCallback(async () => {
    try {
      setError(null);
      // Calls Rust command to read from the in-memory state
      const result = await invoke<PasswordEntry[]>('read_all_entries');
      setEntries(result);
    } catch (e) {
      setError(`Failed to read entries: ${e}`);
    }
  }, []);

  // --- Core Session Management ---

  const loadVault = useCallback(async (masterPassword: string) => {
    setIsLoading(true);
    setError(null);
    try {
      // Calls Rust command to read file, derive key, decrypt vault, and store state
      await invoke('load_and_decrypt_vault', { masterPassword });
      setIsLoggedIn(true);
      
      // Load the entries into the frontend state immediately after successful login
      await readEntries();
    } catch (e) {
      console.error(e);
      setError(e as string);
    } finally {
      setIsLoading(false);
    }
  }, [readEntries]);

  const saveAndLogout = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      // Calls Rust command to re-encrypt in-memory vault, write to disk, and zeroize sensitive memory
      await invoke('save_vault_and_logout');
      
      // Clear all sensitive frontend state after successful logout
      setEntries([]);
      setIsLoggedIn(false);
    } catch (e) {
      setError(`Logout failed: ${e}`);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // --- CRUD Operations ---

  const createEntry = useCallback(async (entry: Omit<PasswordEntry, 'id'>) => {
    try {
      setError(null);
      const newEntry = await invoke<PasswordEntry>('create_entry', { newEntry: entry });
      setEntries(prev => [...prev, newEntry]);
    } catch (e) {
      setError(`Failed to create entry: ${e}`);
    }
  }, []);

  const updateEntry = useCallback(async (entry: PasswordEntry) => {
    try {
      setError(null);
      await invoke('update_entry', { updatedEntry: entry });
      
      // Update the local state
      setEntries(prev => prev.map(e => (e.id === entry.id ? entry : e)));
    } catch (e) {
      setError(`Failed to update entry: ${e}`);
    }
  }, []);
  
  const deleteEntry = useCallback(async (id: string) => {
    try {
      setError(null);
      await invoke('delete_entry', { id });
      
      // Update the local state
      setEntries(prev => prev.filter(e => e.id !== id));
    } catch (e) {
      setError(`Failed to delete entry: ${e}`);
    }
  }, []);

  // Initial read of entries upon successful login (handled within loadVault)
  // useEffect(() => {
  //   if (isLoggedIn) {
  //     readEntries();
  //   }
  // }, [isLoggedIn, readEntries]);


  return {
    entries,
    isLoggedIn,
    isLoading,
    error,
    loadVault,
    saveAndLogout,
    createEntry,
    updateEntry,
    deleteEntry,
    readEntries,
  };
};

// Export the necessary types for consumption
export type { PasswordEntry };