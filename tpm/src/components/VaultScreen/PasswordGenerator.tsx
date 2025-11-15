// src/components/VaultScreen/PasswordGenerator.tsx (UPDATED)

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { GeneratorOptions } from '../../types/backend';

interface PasswordGeneratorProps {
    // New prop: a callback function to handle the generated password
    onPasswordGenerated: (password: string) => void; 
}

const PasswordGenerator: React.FC<PasswordGeneratorProps> = ({ onPasswordGenerated }) => {
    const [password, setPassword] = useState('');
    const [options, setOptions] = useState<GeneratorOptions>({
        length: 16,
        include_symbols: true,
        include_numbers: true,
        include_caps: true,
    });
    const [isLoading, setIsLoading] = useState(false);

    const generatePassword = async () => {
        setIsLoading(true);
        try {
            const result = await invoke<string>('generate_strong_password', { options });
            setPassword(result);
        } catch (e) {
            setPassword(`Error: ${e}`);
        } finally {
            setIsLoading(false);
        }
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value, type, checked } = e.target;
        setOptions(prev => ({
            ...prev,
            [name]: type === 'checkbox' ? checked : parseInt(value) || 0,
        }));
    };
    
    const handleInject = () => {
        if (password && !password.startsWith('Error')) {
            onPasswordGenerated(password);
            // Optionally clear the generator password
            setPassword(''); 
        } else {
            alert("Generate a password first.");
        }
    }

    return (
        <div className="p-4 bg-white dark:bg-gray-800 rounded-lg shadow mt-4 border border-yellow-500">
            <h3 className="text-lg font-semibold mb-3">Password Generator</h3>

            <div className="flex items-center space-x-2 mb-3">
                <input
                    type="text"
                    readOnly
                    value={password}
                    className="flex-grow px-3 py-2 border rounded-lg bg-gray-100 dark:bg-gray-700 font-mono text-sm"
                    placeholder="Click Generate"
                />
                <button
                    onClick={generatePassword}
                    disabled={isLoading || options.length === 0}
                    className="py-2 px-4 bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 disabled:opacity-50 flex-shrink-0"
                >
                    {isLoading ? '...' : 'Generate'}
                </button>
                <button
                    onClick={handleInject}
                    disabled={!password || password.startsWith('Error')}
                    className="py-2 px-4 bg-green-500 text-white rounded-lg hover:bg-green-600 disabled:opacity-50 flex-shrink-0"
                >
                    Inject
                </button>
            </div>

            <div className="grid grid-cols-2 gap-2 text-sm">
                <div className="flex items-center">
                    <label htmlFor="length" className="mr-2">Length ({options.length}):</label>
                    <input
                        id="length"
                        type="range"
                        name="length"
                        min="8"
                        max="64"
                        value={options.length}
                        onChange={handleChange}
                        className="flex-grow"
                    />
                </div>
                {/* Checkbox controls for character sets */}
                {['include_symbols', 'include_numbers', 'include_caps'].map((key) => (
                    <div key={key} className="flex items-center">
                        <input
                            id={key}
                            type="checkbox"
                            name={key}
                            checked={options[key as keyof GeneratorOptions] as boolean}
                            onChange={handleChange}
                            className="mr-2 rounded text-indigo-600 focus:ring-indigo-500"
                        />
                        <label htmlFor={key}>{key.split('_').map(s => s.charAt(0).toUpperCase() + s.slice(1)).join(' ').replace('Caps', 'Caps')}</label>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default PasswordGenerator;