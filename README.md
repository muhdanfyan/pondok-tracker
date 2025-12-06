# Pondok Tracker - Desktop Agent

Desktop Time Tracking Agent untuk Pondok Informatika. Aplikasi ini memantau aktivitas belajar santri di komputer.

## Fitur

- **Window Tracking**: Otomatis merekam aplikasi yang sedang digunakan
- **Idle Detection**: Mendeteksi ketika tidak ada aktivitas
- **System Tray**: Berjalan di background dengan icon di system tray
- **Auto Sync**: Sinkronisasi data ke server setiap 5 menit
- **Cross Platform**: Mendukung Windows, macOS, dan Linux

## Requirements

### Development

- Node.js 18+
- Rust 1.70+
- Platform-specific requirements:
  - **Windows**: Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.0-dev`, `libappindicator3-dev`

### Linux Dependencies

```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf xdotool xprintidle

# Fedora
sudo dnf install webkit2gtk4.0-devel libappindicator-gtk3-devel librsvg2-devel xdotool xprintidle
```

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

## Build Output

Setelah build, file installer akan berada di:

- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

## Usage

1. Download installer sesuai OS
2. Install aplikasi
3. Buka aplikasi dan masukkan token aktivasi
4. Token didapat dari dashboard PISANTRI (Menu Time Tracking > Generate Token)
5. Setelah aktivasi, aplikasi akan berjalan di background

## Architecture

```
├── src/                    # React frontend
│   ├── pages/
│   │   ├── ActivationPage.tsx
│   │   └── TrackingPage.tsx
│   └── App.tsx
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── tracker.rs     # Window tracking
│   │   ├── storage.rs     # Local storage
│   │   ├── api.rs         # API client
│   │   └── tray.rs        # System tray
│   └── Cargo.toml
│
└── package.json
```

## API Endpoints

Agent berkomunikasi dengan backend melalui:

- `POST /api/tracking/agent/activate` - Aktivasi token
- `POST /api/tracking/agent/heartbeat` - Ping status online
- `POST /api/tracking/agent/sync` - Sync aktivitas
- `GET /api/tracking/agent/settings` - Ambil pengaturan

## License

Proprietary - Pondok Informatika
