// src/App.tsx
import { useVault } from './hooks/useVault';
import LoginScreen from './components/LoginScreen.tsx';
import VaultScreen from './components/VaultScreen/VaultScreen.tsx';
import './App.css'; 

function App() {
  const vault = useVault();

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <div className="max-w-7xl mx-auto py-10 px-4 sm:px-6 lg:px-8">
        
        {vault.error && (
          <div className="bg-red-500 text-white p-3 rounded mb-4 font-mono">
            Error: {vault.error}
          </div>
        )}

        {vault.isLoggedIn ? (
          <VaultScreen vault={vault} />
        ) : (
          <LoginScreen 
            isLoading={vault.isLoading} 
            loadVault={vault.loadVault} 
          />
        )}
        
      </div>
    </div>
  );
}

export default App;