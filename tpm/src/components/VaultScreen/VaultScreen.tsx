// src/components/VaultScreen/VaultScreen.tsx (FINAL)

import React, { useState } from 'react';
import { useVault, PasswordEntry } from '../../hooks/useVault';
import PasswordGenerator from './PasswordGenerator';
import EntryForm from './EntryForm'; 

interface VaultScreenProps {
  vault: ReturnType<typeof useVault>;
}

// Entry Row Component (Displays a single entry in the list)
const EntryRow: React.FC<{ entry: PasswordEntry; onEdit: (e: PasswordEntry) => void; onDelete: (id: string) => void }> = ({ entry, onEdit, onDelete }) => {
    const [isPasswordVisible, setIsPasswordVisible] = useState(false);
    return (
        <div className="flex items-center justify-between p-3 mb-2 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
            <div className="truncate flex-grow">
                <p className="font-semibold text-indigo-600 dark:text-indigo-400 truncate">{entry.name}</p>
                <p className="text-sm text-gray-500 dark:text-gray-400 truncate">{entry.username}</p>
            </div>
            <div className="flex items-center space-x-3 flex-shrink-0">
                <button 
                    onClick={() => setIsPasswordVisible(!isPasswordVisible)}
                    className="text-sm text-blue-500 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-200"
                >
                    {isPasswordVisible ? 'Hide' : 'Show'}
                </button>
                <span className="font-mono text-sm w-32 overflow-hidden">
                    {isPasswordVisible ? entry.password : '••••••••••••'}
                </span>
                <button 
                    onClick={() => onEdit(entry)} 
                    className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                >
                    ✏️
                </button>
                <button 
                    onClick={() => onDelete(entry.id)} 
                    className="text-red-500 hover:text-red-700"
                >
                    🗑️
                </button>
            </div>
        </div>
    );
}


const VaultScreen: React.FC<VaultScreenProps> = ({ vault }) => {
  // State holds the entry currently being created/edited
  const [selectedEntry, setSelectedEntry] = useState<Partial<PasswordEntry> | null>(null); 
  const [showGenerator, setShowGenerator] = useState(false);

  const handleEdit = (entry: PasswordEntry) => {
      setSelectedEntry(entry);
  };
  
  const handleNewEntry = () => {
    // Set to a blank object to trigger the new entry form state in EntryForm.tsx
    setSelectedEntry({ 
        name: '', username: '', password: '', 
        url: '', notes: '', tags: []
    }); 
  }

  const handleDelete = (id: string) => {
      if (window.confirm("Are you sure you want to delete this entry? This action cannot be undone.")) {
          vault.deleteEntry(id);
      }
  };

  const handleSaveEntry = (entry: Partial<PasswordEntry>) => {
      // Ensure required fields are not empty (though EntryForm should handle this)
      if (!entry.name || !entry.username || !entry.password) return;

      if (entry.id) {
          // UPDATE: ID is present
          vault.updateEntry(entry as PasswordEntry);
      } else {
          // CREATE: No ID (ID will be generated in Rust)
          // Rust expects Omit<PasswordEntry, 'id'>
          const newEntry: Omit<PasswordEntry, 'id'> = {
              name: entry.name, 
              username: entry.username,
              password: entry.password,
              url: entry.url,
              notes: entry.notes,
              tags: entry.tags,
          };
          vault.createEntry(newEntry);
      }
      setSelectedEntry(null); // Close the form
  };

  const handlePasswordInjection = (password: string) => {
      // Inject the generated password into the currently open form
      if (selectedEntry) {
        setSelectedEntry(prev => ({
            ...prev,
            password: password
        } as Partial<PasswordEntry>));
      }
  }
  
  return (
    <div className="grid grid-cols-3 gap-8">
      
      {/* Sidebar / Controls (Column 1) */}
      <div className="col-span-1 space-y-4">
        <h2 className="text-xl font-bold">Vault Controls</h2>
        
        <button
          onClick={handleNewEntry} 
          className="w-full py-2 px-4 rounded-lg bg-green-600 text-white hover:bg-green-700 disabled:opacity-50"
          disabled={!!selectedEntry} 
        >
          + Add New Entry
        </button>

        <button
          onClick={() => setShowGenerator(!showGenerator)}
          className="w-full py-2 px-4 rounded-lg bg-yellow-600 text-white hover:bg-yellow-700"
        >
          {showGenerator ? 'Hide Generator' : 'Show Password Generator'}
        </button>

        <button
          onClick={vault.saveAndLogout}
          className="w-full py-2 px-4 rounded-lg bg-red-600 text-white hover:bg-red-700"
          disabled={vault.isLoading}
        >
          {vault.isLoading ? 'Saving & Logging Out...' : 'Save & Logout'}
        </button>
        
        {/* Pass the injection handler to the generator */}
        {showGenerator && <PasswordGenerator onPasswordGenerated={handlePasswordInjection} />}
      </div>
      
      {/* Main Content Area (Column 2) */}
      <div className="col-span-2">
        <h2 className="text-2xl font-bold mb-4">Your Vault ({vault.entries.length} Entries)</h2>

        {/* Entry Form (Create/Update) */}
        {selectedEntry && (
            <div className="mb-6 border-2 border-indigo-500 p-6 rounded-lg bg-white dark:bg-gray-800 shadow-xl">
                <EntryForm
                    initialEntry={selectedEntry}
                    onSave={handleSaveEntry}
                    onCancel={() => setSelectedEntry(null)}
                    // When the form's generate button is clicked, we show the main generator
                    onGeneratePassword={() => setShowGenerator(true)} 
                />
            </div>
        )}

        {/* Entry List */}
        <div className="space-y-3">
          {vault.entries.length === 0 ? (
            <p className="text-gray-500 dark:text-gray-400 p-4 border rounded-lg bg-white dark:bg-gray-800">
                Your vault is currently empty. Click **'+ Add New Entry'** to get started!
            </p>
          ) : (
            vault.entries.map((entry) => (
              <EntryRow 
                  key={entry.id} 
                  entry={entry} 
                  onEdit={handleEdit} 
                  onDelete={handleDelete}
              />
            ))
          )}
        </div>
      </div>
    </div>
  );
};

export default VaultScreen;