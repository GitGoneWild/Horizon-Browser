# Horizon Browser - Feature Documentation

## Overview

Horizon Browser is a feature-rich, privacy-focused web browser built in Rust. This document provides detailed information about all available features.

## Core Features

### Multi-Tab Browsing
- Create, close, and switch between tabs
- Loading indicators for active tabs
- Tab history with back/forward navigation
- Keyboard shortcuts for tab management (Ctrl+T, Ctrl+W)

### Smart Address Bar
- **SSL/Security Indicators**: Visual indicators for HTTPS (🔒), HTTP (⚠), and internal pages (ℹ)
- **Intelligent URL Detection**: Automatically distinguishes between URLs and search queries
- **Multiple Search Engines**: DuckDuckGo (default), Google, Bing, Brave Search
- **URL Normalization**: Automatic HTTPS upgrade for domain entries

### Privacy & Security
- **Tracking Protection**: Blocks known trackers (enabled by default)
- **Do Not Track**: Sends DNT header to websites (enabled by default)
- **Third-Party Cookie Blocking**: Prevents cross-site tracking (enabled by default)
- **HTTPS-Only Mode**: Optional enforcement of secure connections
- **Clear Data on Exit**: Automatic cleanup when browser closes

### Network Features

#### DNS Control
- System Default DNS
- Google DNS (8.8.8.8, 8.8.4.4)
- Cloudflare DNS (1.1.1.1, 1.0.0.1)
- Quad9 DNS (9.9.9.9)
- Custom DNS servers
- Changes take effect immediately

#### VPN Management
- **HTTP/HTTPS Proxy Support**: Simple proxy configuration
- **SOCKS5 Proxy**: Advanced proxy with authentication
- **OpenVPN Support**: Full .ovpn file compatibility
- Per-browser VPN (only browser traffic routed through VPN)
- Real-time VPN statistics (coming soon)

#### Network Speed Test
- Download/upload speed measurement
- Ping latency testing
- Jitter calculation
- Test server location display
- Accessible from Network settings

### Password Management
- Secure local password storage
- Multiple accounts per website support
- Password search functionality
- Auto-fill suggestions (coming soon)
- URL normalization for password matching
- Master password protection (coming soon)

### Extension System
- **Firefox WebExtensions Compatible**: Full Firefox extension API support
- **Manifest v2 and v3**: Latest extension standards
- **Background Scripts**: Extensions can run in the background
- **Content Scripts**: Inject JavaScript into web pages
- **Browser Actions**: Toolbar buttons for extensions
- **Standard Permissions**: tabs, storage, bookmarks, cookies, webRequest, etc.
- Install from repository or .xpi files
- Automatic extension updates

### Customization
- **Themes**: Dark (default) and Light themes
- **Font Size**: Adjustable from 10-20px
- **Homepage**: Customizable start page
- **Bookmarks Bar**: Toggle visibility
- **Search Engine**: Choice of 4 search providers

### Advanced Features
- **Developer Tools**: Coming soon (DOM inspection, console, network monitor)
- **Hardware Acceleration**: GPU-accelerated rendering (enabled by default)
- **Experimental Features**: Opt-in to test new features

## Keyboard Shortcuts

- `Ctrl+T`: New tab
- `Ctrl+W`: Close tab
- `Ctrl+R` / `F5`: Reload page
- `Alt+Left`: Go back
- `Alt+Right`: Go forward
- `Alt+Home`: Go to homepage
- `Ctrl+L`: Focus address bar

## Settings Panels

1. **General**: Homepage, search engine, startup options
2. **Privacy**: Tracking protection, DNT, cookies, HTTPS
3. **Appearance**: Theme, font size, bookmarks bar
4. **Network & VPN**: DNS, VPN, speed test
5. **Passwords**: Password manager, auto-fill options
6. **Extensions**: Extension management, permissions
7. **Downloads**: Download directory, save preferences
8. **Advanced**: Developer tools, hardware acceleration, experimental features

## Special Pages

- `about:home`: Browser home page
- `about:settings`: Settings and preferences
- `about:blank`: Blank page

## Platform Support

- **Windows**: Windows 10 and later
- **macOS**: macOS 10.15 (Catalina) and later
- **Linux**: Most modern distributions

## Privacy First

- No telemetry or analytics
- All data stored locally
- Privacy features enabled by default
- Open source and auditable

For more information, see the full documentation in the [docs](.) directory.
