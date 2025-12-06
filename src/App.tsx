import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import ActivationPage from './pages/ActivationPage';
import TrackingPage from './pages/TrackingPage';

interface AppState {
  isActivated: boolean;
  santriId: number | null;
  santriName: string;
  token: string;
}

function App() {
  const [state, setState] = useState<AppState>({
    isActivated: false,
    santriId: null,
    santriName: '',
    token: '',
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    checkActivation();
  }, []);

  const checkActivation = async () => {
    try {
      const result = await invoke<{ is_activated: boolean; santri_id: number | null; santri_name: string; token: string }>('check_activation');
      setState({
        isActivated: result.is_activated,
        santriId: result.santri_id,
        santriName: result.santri_name,
        token: result.token,
      });
    } catch (error) {
      console.error('Failed to check activation:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleActivated = (santriId: number, santriName: string, token: string) => {
    setState({
      isActivated: true,
      santriId,
      santriName,
      token,
    });
  };

  if (loading) {
    return (
      <div className="container" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: '100vh' }}>
        <div style={{ textAlign: 'center' }}>
          <div style={{ fontSize: 48, marginBottom: 16 }}>🏫</div>
          <p>Memuat...</p>
        </div>
      </div>
    );
  }

  if (!state.isActivated) {
    return <ActivationPage onActivated={handleActivated} />;
  }

  return (
    <TrackingPage 
      santriId={state.santriId!} 
      santriName={state.santriName} 
      token={state.token}
    />
  );
}

export default App;
