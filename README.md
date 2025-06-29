# 🌍 IPStatus

A Tauri-based desktop application that automatically detects your public IP address and displays the corresponding country flag in the system tray.

![IPStatus Screenshot](https://via.placeholder.com/600x400?text=IPStatus)

## ✨ Features

- 🔍 **Automatic IP Detection**: Detects your public IP address in real-time
- 🏳️ **Country Flag Display**: Shows the corresponding country flag in the system tray
- 🌍 **Fallback Support**: Uses Earth icon when location cannot be determined
- 🚀 **Lightweight**: Built with Tauri for minimal resource usage
- 🔒 **Privacy Focused**: No data collection, purely local processing
- ⚡ **Fast Updates**: Real-time location detection and updates

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version)
- [Rust](https://rustup.rs/) (latest stable)

### Development

1. Clone the repository:
```bash
git clone https://github.com/yourusername/IPStatus.git
cd IPStatus
```

2. Install dependencies:
```bash
npm install
```

3. Run in development mode:
```bash
npm run dev
```

4. Build for production:
```bash
npm run build
```

## 📦 Download

### Stable Releases

Download the latest stable release from the [Releases](https://github.com/yourusername/IPStatus/releases) page:

- **macOS (Apple Silicon)**: `IPStatus_aarch64.dmg`
- **Windows (x64)**: `IPStatus_x64.msi`

### Development Builds

Development builds are automatically created for every commit to the `develop` branch and can be found in the [Actions](https://github.com/yourusername/IPStatus/actions) tab.

## 🏗️ Build Process

This project uses GitHub Actions for automated building:

- **Production builds**: Triggered on tags (`v*`)
- **Development builds**: Triggered on pushes to `main` and `develop` branches
- **Supported platforms**: macOS (ARM64) and Windows (x64)

## 🛠️ Technology Stack

- **Frontend**: HTML, CSS, JavaScript
- **Backend**: Rust with Tauri 2.0
- **UI Framework**: Native HTML/CSS with modern design
- **Build System**: GitHub Actions
- **Package Manager**: npm

## 🎯 How It Works

1. **IP Detection**: Uses multiple IP detection services for reliability:
   - ip-api.com
   - ipapi.co
   - ipify.org

2. **Location Mapping**: Maps country codes to Unicode flag emojis

3. **Tray Integration**: Updates system tray icon and tooltip with country information

4. **Error Handling**: Falls back to Earth icon if detection fails

## 🔧 Configuration

The application uses the following configuration files:

- `src-tauri/tauri.conf.json`: Tauri application configuration
- `src-tauri/Cargo.toml`: Rust dependencies and build settings
- `package.json`: Node.js dependencies and scripts

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Bug Reports

If you find a bug, please create an issue on the [Issues](https://github.com/yourusername/IPStatus/issues) page with:

- Your operating system and version
- Steps to reproduce the bug
- Expected vs actual behavior
- Screenshots if applicable

## ⭐ Support

If you find this project useful, please consider:

- ⭐ Starring the repository
- 🐛 Reporting bugs
- 💡 Suggesting new features
- 🤝 Contributing code

## 📞 Contact

- GitHub: [@yourusername](https://github.com/yourusername)
- Issues: [Project Issues](https://github.com/yourusername/IPStatus/issues)

---

Built with ❤️ using [Tauri](https://tauri.app/)
