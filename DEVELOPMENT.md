# 🛠️ Development Guide

## Project Structure

```
NoChinaIP/
├── src/                    # Frontend code
│   ├── index.html         # Main HTML file
│   ├── styles.css         # Application styles
│   ├── main.js           # Frontend JavaScript
│   └── assets/           # Static assets
├── src-tauri/            # Tauri backend
│   ├── src/
│   │   └── main.rs       # Rust main application
│   ├── icons/            # Application icons
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── .github/
│   └── workflows/
│       └── build.yml     # GitHub Actions workflow
├── package.json          # Node.js dependencies
└── README.md            # Project documentation
```

## Key Features

### IP Detection
- Uses multiple IP services for reliability:
  - ip-api.com (primary)
  - ipapi.co (fallback)
  - ipify.org (fallback)

### Country Flag Display
- Maps country codes to Unicode flag emojis
- Supports 50+ countries
- Falls back to Earth emoji (🌍) for unknown locations

### System Tray Integration
- Shows country flag in system tray
- Click to open/focus main window
- Tooltip displays country name and flag

### Auto-refresh
- Checks IP location every 5 minutes
- Manual refresh button available
- Real-time updates to tray icon

## Development Commands

```bash
# Install dependencies
npm install

# Run in development mode (with hot reload)
npm run dev

# Build for production
npm run build

# Check Rust code
cargo check --manifest-path src-tauri/Cargo.toml

# Run tests
cargo test --manifest-path src-tauri/Cargo.toml
```

## Architecture

### Frontend (HTML/CSS/JS)
- Modern responsive design
- Real-time IP information display
- Manual refresh capability
- Clean, intuitive interface

### Backend (Rust + Tauri)
- Async HTTP requests for IP detection
- System tray management
- Cross-platform compatibility
- Efficient resource usage

### Build System
- GitHub Actions for CI/CD
- Multi-platform builds (macOS ARM, Windows x64)
- Automated releases on version tags

## Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)
- Application metadata
- Window settings (hidden by default)
- Bundle configuration
- Security settings

### Rust Dependencies (`src-tauri/Cargo.toml`)
- `tauri`: Main framework with tray icon support
- `reqwest`: HTTP client for API calls
- `serde`: JSON serialization/deserialization
- `tokio`: Async runtime

## Adding New Countries

To add support for a new country flag:

1. Add the country code mapping in `get_flag_emoji()` function in `src-tauri/src/main.rs`:

```rust
"XX" => "🇽🇽".to_string(),  // Replace XX with country code
```

2. The flag emoji follows the pattern: Regional Indicator Symbols for the two-letter country code.

## Debugging

### Frontend
- Open browser developer tools when window is visible
- Console logs for IP detection results
- Network tab for API call monitoring

### Backend
- Check terminal output for Rust logs
- Use `cargo run --manifest-path src-tauri/Cargo.toml` for direct execution
- Enable debug logging with `RUST_LOG=debug`

## Known Limitations

1. **IP Detection Accuracy**: Depends on third-party services
2. **Firewall/Proxy**: May affect IP detection in corporate environments
3. **Rate Limiting**: API services may have usage limits
4. **Platform Differences**: Tray icon behavior varies by OS

## Deployment

### GitHub Actions
- Automatically builds on push to `main` and `develop`
- Creates releases for version tags (`v*`)
- Supports macOS (ARM64) and Windows (x64)

### Manual Build
```bash
# Build for current platform
npm run build

# Build for specific target (from correct OS)
npm run tauri build -- --target aarch64-apple-darwin  # macOS ARM
npm run tauri build -- --target x86_64-pc-windows-msvc # Windows x64
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### Code Style
- Follow Rust conventions for backend code
- Use consistent indentation (2 spaces for HTML/CSS/JS)
- Add comments for complex logic
- Update documentation for new features

## Troubleshooting

### Common Issues

1. **Build Failures**
   - Ensure Rust and Node.js are installed
   - Check that all dependencies are up to date
   - Verify platform-specific requirements

2. **Tray Icon Not Appearing**
   - Check OS-specific tray icon requirements
   - Verify application permissions
   - Test with elevated privileges if needed

3. **IP Detection Fails**
   - Check internet connectivity
   - Verify API service availability
   - Test with different networks

For more issues, check the [GitHub Issues](https://github.com/yourusername/NoChinaIP/issues) page. 