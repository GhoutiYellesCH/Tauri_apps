// src/types/backend.ts

// Matches the Rust struct for KDF parameters
export interface KdfParams {
  memory_cost: number;
  time_cost: number;
  parallelism: number;
}

// Matches the Rust struct for a single password entry
// Note: This data is only available in plaintext in the front-end memory while logged in.
export interface PasswordEntry {
  id: string; // Uuid generated in Rust
  name: string;
  username: string;
  password: string; // Plaintext password
  url?: string;
  notes?: string;
  tags?: string[];
}

// Matches the Rust struct for the password generator options
export interface GeneratorOptions {
  length: number;
  include_symbols: boolean;
  include_numbers: boolean;
  include_caps: boolean;
}

// Initial state for KDF Params when creating a vault
export const DEFAULT_KDF_PARAMS: KdfParams = {
  memory_cost: 65536, // 64 MiB
  time_cost: 4,
  parallelism: 1,
};