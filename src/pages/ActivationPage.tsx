import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  onActivated: (santriId: number, santriName: string, token: string) => void;
}

export default function ActivationPage({ onActivated }: Props) {
  const [token, setToken] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleActivate = async () => {
    if (!token.trim()) {
      setError('Masukkan token aktivasi');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const result = await invoke<{ success: boolean; santri_id: number; santri_name: string; message: string }>('activate_token', { token: token.trim() });
      
      if (result.success) {
        onActivated(result.santri_id, result.santri_name, token.trim());
      } else {
        setError(result.message || 'Token tidak valid');
      }
    } catch (err: any) {
      setError(err.message || 'Gagal mengaktifkan token');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: '100vh' }}>
      <div className="card" style={{ width: '100%' }}>
        <div style={{ textAlign: 'center', marginBottom: 24 }}>
          <div style={{ fontSize: 64, marginBottom: 16 }}>🏫</div>
          <h1 style={{ fontSize: 24, fontWeight: 700, marginBottom: 8 }}>Pondok Tracker</h1>
          <p style={{ color: 'var(--gray-500)' }}>Aktivasi agent untuk memulai tracking</p>
        </div>

        <div style={{ marginBottom: 16 }}>
          <label className="label">Token Aktivasi</label>
          <input
            type="text"
            className="input"
            placeholder="PI-2025-XXX-XXXXXXXX"
            value={token}
            onChange={(e) => setToken(e.target.value.toUpperCase())}
            onKeyDown={(e) => e.key === 'Enter' && handleActivate()}
          />
          {error && (
            <p style={{ color: 'var(--danger)', fontSize: 14, marginTop: 8 }}>{error}</p>
          )}
        </div>

        <button 
          className="btn btn-primary" 
          style={{ width: '100%' }}
          onClick={handleActivate}
          disabled={loading}
        >
          {loading ? 'Mengaktifkan...' : '🔓 Aktivasi'}
        </button>

        <div style={{ marginTop: 24, padding: 16, background: 'var(--gray-50)', borderRadius: 8 }}>
          <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 8 }}>Cara mendapatkan token:</h3>
          <ol style={{ fontSize: 13, color: 'var(--gray-500)', paddingLeft: 16 }}>
            <li>Login ke PISANTRI di browser</li>
            <li>Buka menu "Time Tracking"</li>
            <li>Klik "Generate Token"</li>
            <li>Salin token dan paste di sini</li>
          </ol>
        </div>
      </div>
    </div>
  );
}
