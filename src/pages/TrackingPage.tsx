import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  santriId: number;
  santriName: string;
  token: string;
}

interface TrackingState {
  status: 'standby' | 'active' | 'paused' | 'idle';
  trackingId: number | null;
  duration: number;
  productiveDuration: number;
  idleDuration: number;
  currentApp: string;
  currentWindow: string;
}

interface AppUsage {
  name: string;
  duration: number;
  category: 'productive' | 'neutral' | 'unproductive';
}

export default function TrackingPage({ santriId, santriName, token }: Props) {
  const [state, setState] = useState<TrackingState>({
    status: 'standby',
    trackingId: null,
    duration: 0,
    productiveDuration: 0,
    idleDuration: 0,
    currentApp: '',
    currentWindow: '',
  });
  const [appUsage, setAppUsage] = useState<AppUsage[]>([]);
  const [showStartForm, setShowStartForm] = useState(false);
  const [showEndForm, setShowEndForm] = useState(false);
  const [rencana, setRencana] = useState('');
  const [hasil, setHasil] = useState('');
  const [kendala, setKendala] = useState('');

  // Format duration
  const formatDuration = (seconds: number) => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  // Fetch current state from Rust backend
  const fetchState = useCallback(async () => {
    try {
      const result = await invoke<TrackingState>('get_tracking_state');
      setState(result);
    } catch (error) {
      console.error('Failed to fetch state:', error);
    }
  }, []);

  // Fetch app usage
  const fetchAppUsage = useCallback(async () => {
    try {
      const result = await invoke<AppUsage[]>('get_app_usage');
      setAppUsage(result);
    } catch (error) {
      console.error('Failed to fetch app usage:', error);
    }
  }, []);

  useEffect(() => {
    fetchState();
    fetchAppUsage();
    
    // Poll every second for timer update
    const interval = setInterval(() => {
      if (state.status === 'active') {
        setState(prev => ({ ...prev, duration: prev.duration + 1 }));
      }
      fetchState();
      fetchAppUsage();
    }, 1000);

    return () => clearInterval(interval);
  }, [state.status]);

  const handleStart = async () => {
    if (!rencana.trim()) return;
    
    try {
      const result = await invoke<{ success: boolean; tracking_id: number }>('start_tracking', { 
        rencanaBelajar: rencana,
        token 
      });
      
      if (result.success) {
        setState(prev => ({ 
          ...prev, 
          status: 'active', 
          trackingId: result.tracking_id,
          duration: 0 
        }));
        setShowStartForm(false);
        setRencana('');
      }
    } catch (error) {
      console.error('Failed to start tracking:', error);
    }
  };

  const handlePause = async () => {
    try {
      await invoke('pause_tracking');
      setState(prev => ({ ...prev, status: 'paused' }));
    } catch (error) {
      console.error('Failed to pause:', error);
    }
  };

  const handleResume = async () => {
    try {
      await invoke('resume_tracking');
      setState(prev => ({ ...prev, status: 'active' }));
    } catch (error) {
      console.error('Failed to resume:', error);
    }
  };

  const handleEnd = async () => {
    try {
      await invoke('end_tracking');
      setShowEndForm(true);
    } catch (error) {
      console.error('Failed to end:', error);
    }
  };

  const handleSubmit = async () => {
    if (!hasil.trim()) return;
    
    try {
      await invoke('submit_report', { 
        hasilBelajar: hasil,
        kendala: kendala || null,
        token
      });
      
      setState(prev => ({ 
        ...prev, 
        status: 'standby', 
        trackingId: null,
        duration: 0,
        productiveDuration: 0,
        idleDuration: 0
      }));
      setShowEndForm(false);
      setHasil('');
      setKendala('');
    } catch (error) {
      console.error('Failed to submit:', error);
    }
  };

  const getStatusBadge = () => {
    const badges = {
      standby: { class: 'status-standby', icon: '⚪', text: 'Standby' },
      active: { class: 'status-active', icon: '🟢', text: 'Tracking' },
      paused: { class: 'status-paused', icon: '⏸️', text: 'Paused' },
      idle: { class: 'status-idle', icon: '🟡', text: 'Idle' },
    };
    const badge = badges[state.status];
    return (
      <span className={`status-badge ${badge.class}`}>
        {badge.icon} {badge.text}
      </span>
    );
  };

  const getCategoryColor = (category: string) => {
    if (category === 'productive') return 'var(--success)';
    if (category === 'unproductive') return 'var(--danger)';
    return 'var(--gray-400)';
  };

  return (
    <div className="container">
      {/* Header */}
      <div className="card">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <h2 style={{ fontSize: 18, fontWeight: 600 }}>{santriName}</h2>
            <p style={{ fontSize: 13, color: 'var(--gray-500)' }}>ID: {santriId}</p>
          </div>
          {getStatusBadge()}
        </div>
      </div>

      {/* Timer Card */}
      <div className="card">
        <div className="timer">{formatDuration(state.duration)}</div>
        <div className="timer-label">
          {state.status === 'active' ? `Sedang tracking: ${state.currentApp}` : 
           state.status === 'paused' ? 'Tracking dipausa' : 
           'Belum mulai tracking'}
        </div>

        <div className="stats-grid">
          <div className="stat-item">
            <div className="stat-value" style={{ color: 'var(--success)' }}>
              {formatDuration(state.productiveDuration).slice(0, 5)}
            </div>
            <div className="stat-label">Produktif</div>
          </div>
          <div className="stat-item">
            <div className="stat-value" style={{ color: 'var(--warning)' }}>
              {formatDuration(state.idleDuration).slice(0, 5)}
            </div>
            <div className="stat-label">Idle</div>
          </div>
          <div className="stat-item">
            <div className="stat-value">
              {state.duration > 0 ? Math.round((state.productiveDuration / state.duration) * 100) : 0}%
            </div>
            <div className="stat-label">Produktivitas</div>
          </div>
        </div>
      </div>

      {/* Control Buttons */}
      <div style={{ display: 'flex', gap: 12, marginBottom: 16 }}>
        {state.status === 'standby' && (
          <button 
            className="btn btn-primary" 
            style={{ flex: 1 }}
            onClick={() => setShowStartForm(true)}
          >
            ▶️ Mulai Tracking
          </button>
        )}
        {state.status === 'active' && (
          <>
            <button className="btn btn-warning" style={{ flex: 1 }} onClick={handlePause}>
              ⏸️ Pause
            </button>
            <button className="btn btn-danger" style={{ flex: 1 }} onClick={handleEnd}>
              ⏹️ Selesai
            </button>
          </>
        )}
        {state.status === 'paused' && (
          <>
            <button className="btn btn-success" style={{ flex: 1 }} onClick={handleResume}>
              ▶️ Lanjut
            </button>
            <button className="btn btn-danger" style={{ flex: 1 }} onClick={handleEnd}>
              ⏹️ Selesai
            </button>
          </>
        )}
      </div>

      {/* App Usage */}
      {appUsage.length > 0 && (
        <div className="card">
          <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>Aplikasi Digunakan</h3>
          <div className="app-list">
            {appUsage.slice(0, 5).map((app, i) => (
              <div key={i} className="app-item">
                <div className="app-icon">
                  {app.category === 'productive' ? '✅' : 
                   app.category === 'unproductive' ? '⚠️' : '📱'}
                </div>
                <div className="app-info">
                  <div className="app-name">{app.name}</div>
                  <div className="app-duration">{formatDuration(app.duration)}</div>
                </div>
                <div style={{ width: 60 }}>
                  <div className="progress-bar">
                    <div 
                      className="progress-fill"
                      style={{ 
                        width: `${(app.duration / state.duration) * 100}%`,
                        background: getCategoryColor(app.category)
                      }}
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Start Form Modal */}
      {showStartForm && (
        <div style={{
          position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.5)',
          display: 'flex', alignItems: 'center', justifyContent: 'center', padding: 24
        }}>
          <div className="card" style={{ width: '100%', maxWidth: 400 }}>
            <h3 style={{ fontSize: 18, fontWeight: 600, marginBottom: 16 }}>Mulai Tracking</h3>
            
            <div style={{ marginBottom: 16 }}>
              <label className="label">Apa yang akan dipelajari?</label>
              <textarea 
                className="input textarea"
                placeholder="Contoh: Belajar React Hooks..."
                value={rencana}
                onChange={(e) => setRencana(e.target.value)}
              />
            </div>

            <div style={{ display: 'flex', gap: 12 }}>
              <button 
                className="btn btn-outline" 
                style={{ flex: 1 }}
                onClick={() => setShowStartForm(false)}
              >
                Batal
              </button>
              <button 
                className="btn btn-primary" 
                style={{ flex: 1 }}
                onClick={handleStart}
                disabled={!rencana.trim()}
              >
                Mulai
              </button>
            </div>
          </div>
        </div>
      )}

      {/* End Form Modal */}
      {showEndForm && (
        <div style={{
          position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.5)',
          display: 'flex', alignItems: 'center', justifyContent: 'center', padding: 24
        }}>
          <div className="card" style={{ width: '100%', maxWidth: 400 }}>
            <h3 style={{ fontSize: 18, fontWeight: 600, marginBottom: 16 }}>Laporan Hari Ini</h3>
            
            <div style={{ marginBottom: 16 }}>
              <label className="label">Apa yang sudah dipelajari?</label>
              <textarea 
                className="input textarea"
                placeholder="Tuliskan hasil belajar..."
                value={hasil}
                onChange={(e) => setHasil(e.target.value)}
              />
            </div>

            <div style={{ marginBottom: 16 }}>
              <label className="label">Kendala (opsional)</label>
              <textarea 
                className="input textarea"
                placeholder="Kendala yang dihadapi..."
                value={kendala}
                onChange={(e) => setKendala(e.target.value)}
                style={{ minHeight: 60 }}
              />
            </div>

            <button 
              className="btn btn-primary" 
              style={{ width: '100%' }}
              onClick={handleSubmit}
              disabled={!hasil.trim()}
            >
              📤 Kirim Laporan
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
