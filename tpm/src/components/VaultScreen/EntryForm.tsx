// src/components/VaultScreen/EntryForm.tsx

import React, { useState, useEffect } from 'react';
import { PasswordEntry } from '../../hooks/useVault';

// Define the shape for form state: a partial entry, where 'id' is optional for new entries
type FormEntryState = Partial<PasswordEntry>; 

interface EntryFormProps {
  initialEntry: FormEntryState;
  onSave: (entry: FormEntryState) => void;
  onCancel: () => void;
  // Callback to signal the parent to generate a password
  onGeneratePassword: () => void;
}

const EntryForm: React.FC<EntryFormProps> = ({ 
  initialEntry, 
  onSave, 
  onCancel,
  onGeneratePassword 
}) => {
  const isNew = !initialEntry.id;
  
  // Initialize form state
  const [entry, setEntry] = useState<FormEntryState>(() => ({
    name: initialEntry.name || '',
    username: initialEntry.username || '',
    password: initialEntry.password || '',
    url: initialEntry.url || '',
    notes: initialEntry.notes || '',
    tags: initialEntry.tags || [],
    id: initialEntry.id,
  }));

  // Update state if the initialEntry prop changes (e.g., user clicks 'Edit' on a different entry)
  useEffect(() => {
    setEntry(initialEntry);
  }, [initialEntry]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setEntry(prev => ({ ...prev, [name]: value }));
  };
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Basic validation
    if (!entry.name || !entry.username || !entry.password) {
      alert('Name, Username, and Password are required fields.');
      return;
    }
    onSave(entry); 
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <h3 className="text-xl font-bold text-indigo-600 dark:text-indigo-400">
        {isNew ? 'Create New Entry' : `Edit Entry: ${entry.name}`}
      </h3>

      {/* Name */}
      <div>
        <label htmlFor="name" className="block text-sm font-medium text-gray-700 dark:text-gray-300">Name (Required)</label>
        <input
          id="name"
          type="text"
          name="name"
          value={entry.name}
          onChange={handleChange}
          required
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm dark:bg-gray-700 dark:border-gray-600 p-2"
        />
      </div>

      {/* Username */}
      <div>
        <label htmlFor="username" className="block text-sm font-medium text-gray-700 dark:text-gray-300">Username (Required)</label>
        <input
          id="username"
          type="text"
          name="username"
          value={entry.username}
          onChange={handleChange}
          required
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm dark:bg-gray-700 dark:border-gray-600 p-2"
        />
      </div>

      {/* Password */}
      <div>
        <label htmlFor="password" className="block text-sm font-medium text-gray-700 dark:text-gray-300">Password (Required)</label>
        <div className="flex space-x-2">
            <input
                id="password"
                type="text"
                name="password"
                value={entry.password}
                onChange={handleChange}
                required
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm dark:bg-gray-700 dark:border-gray-600 p-2"
            />
            <button 
                type="button" 
                onClick={onGeneratePassword} 
                className="py-1 px-3 text-sm bg-yellow-500 hover:bg-yellow-600 text-white rounded-md flex-shrink-0"
            >
                Generate
            </button>
        </div>
      </div>
      
      {/* URL */}
      <div>
        <label htmlFor="url" className="block text-sm font-medium text-gray-700 dark:text-gray-300">URL (Optional)</label>
        <input
          id="url"
          type="url"
          name="url"
          value={entry.url}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm dark:bg-gray-700 dark:border-gray-600 p-2"
        />
      </div>
      
      {/* Notes */}
      <div>
        <label htmlFor="notes" className="block text-sm font-medium text-gray-700 dark:text-gray-300">Notes (Optional)</label>
        <textarea
          id="notes"
          name="notes"
          value={entry.notes}
          onChange={handleChange}
          rows={3}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm dark:bg-gray-700 dark:border-gray-600 p-2"
        />
      </div>

      <div className="flex justify-end space-x-3 pt-2">
        <button
          type="button"
          onClick={onCancel}
          className="py-2 px-4 rounded-lg text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700"
        >
          Cancel
        </button>
        <button
          type="submit"
          className="py-2 px-4 rounded-lg bg-indigo-600 text-white hover:bg-indigo-700"
        >
          {isNew ? 'Create Entry' : 'Save Changes'}
        </button>
      </div>
    </form>
  );
};

export default EntryForm;