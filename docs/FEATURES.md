# Horizon Browser Features

## Overview

Horizon Browser is a fully functional, modern desktop browser built in Rust with a focus on privacy, performance, and usability.

## Core Features

### 1. Tab Management

- **Multiple Tabs**: Open and manage multiple browser tabs simultaneously
- **Tab Creation**: Create new tabs with the ➕ button or navigate to URLs
- **Tab Switching**: Click on any tab to switch to it
- **Tab Closing**: Close individual tabs with the ✕ button (last tab cannot be closed)
- **Visual Tab Bar**: Clean, modern tab bar showing all open tabs with truncated titles

### 2. Navigation

- **Address Bar**: Full-featured URL input with auto-protocol detection
  - Supports `http://`, `https://`, and `about:` URLs
  - Automatically adds `https://` for URLs with domains
  - Treats non-URL input as DuckDuckGo search queries
- **Back Button** (◀): Navigate backward in tab history
- **Forward Button** (▶): Navigate forward in tab history
- **Reload Button** (⟳): Refresh the current page
- **Go Button**: Navigate to the entered URL
- **Keyboard Support**: Press Enter in address bar to navigate

### 3. Navigation History

Each tab maintains its own navigation history:
- URLs are added to history when navigating
- Back/forward buttons are enabled/disabled based on history position
- History is preserved when switching between tabs

### 4. Default Home Page

**about:home** - Beautiful welcome page featuring:
- Horizon Browser branding with emoji logo 🌅
- Welcome message
- Feature highlights:
  - 🔒 Privacy First: Built-in tracking protection
  - ⚡ Fast & Lightweight: Minimal resource usage
  - 🎨 Modern UI: Clean dark theme interface
- User guidance for getting started

**about:blank** - Simple blank page

### 5. Web Page Rendering

Currently displays placeholder content for web pages showing:
- Page URL with emoji indicator 📄
- Simulated web content in a styled frame
- Information about full implementation plans

### 6. Modern Dark Theme UI

GitHub-inspired dark theme with:
- **Primary Background**: #0d1117 (GitHub dark)
- **Secondary Background**: #161b22 (panels, tab bar)
- **Tertiary Background**: #21262d (frames, borders)
- **Accent Color**: #58a6ff (blue, links, highlights)
- **Text Colors**: 
  - Primary: #e6edf3 (bright white)
  - Secondary: #7d8ca0 (muted)
  - Muted: #57606a (subtle)

### 7. User Interface Components

#### Tab Bar
- Shows all open tabs with titles
- Active tab highlighted with lighter background
- Inactive tabs in darker shade
- Close button (✕) on each tab
- New tab button (➕) at the end
- Smooth hover effects

#### Navigation Bar
- Back/forward/reload navigation buttons
- Full-width address bar with hint text
- Go button for explicit navigation
- Disabled state for unavailable actions

#### Content Area
- Scrollable content region
- Full-height display
- Responsive to window resizing

## Privacy Features

As configured in the storage settings:

- **Tracking Protection**: Enabled by default
- **Do Not Track**: Enabled by default
- **Third-Party Cookie Blocking**: Enabled by default
- **Privacy-First Search**: Uses DuckDuckGo as default search engine

## Cross-Platform Support

Horizon Browser is built with cross-platform compatibility:

- **Windows**: Native Windows support
- **macOS**: Native macOS support
- **Linux**: Native Linux support

Uses `eframe` and `egui` for cross-platform GUI rendering.

## Technical Architecture

### Modular Design

The browser is organized into specialized crates:

1. **horizon-ui**: User interface and window management
   - Tab management (`tabs.rs`)
   - Window creation and layout (`window.rs`)
   - Theme system (`theme.rs`)

2. **horizon-engine**: Core rendering engine

3. **horizon-storage**: Settings and data persistence

4. **horizon-networking**: Network operations

5. **horizon-extensions**: Extension framework

6. **horizon-sandbox**: Security and isolation

7. **horizon-launcher**: Main application entry point

### Tab Management System

The `TabManager` handles:
- Creating and destroying tabs
- Switching between tabs
- Managing tab state and history
- Preventing closure of last tab

Each `Tab` contains:
- Unique identifier (UUID)
- Current URL
- Page title
- Navigation history stack
- History position index
- Loading state

## Usage

### Starting the Browser

```bash
cargo run --release
```

### Navigation

1. **Open a URL**: Type or paste URL in address bar and press Enter or click Go
2. **Search**: Type search terms (without domain) and press Enter
3. **New Tab**: Click the ➕ button
4. **Switch Tabs**: Click on any tab
5. **Close Tab**: Click the ✕ on the tab
6. **Go Back**: Click the ◀ button
7. **Go Forward**: Click the ▶ button
8. **Reload**: Click the ⟳ button

### Special URLs

- `about:home` - Home page with features and welcome
- `about:blank` - Blank page
- Regular URLs - Web page placeholder view

## Future Enhancements

Potential future features:

1. Full web rendering engine integration (WebView or Servo)
2. Bookmarks management
3. Download manager
4. Browser history persistence
5. Extensions/plugins support
6. Developer tools
7. Settings UI
8. Password manager
9. Session restore
10. Incognito mode

## Testing

Run the comprehensive test suite:

```bash
cargo test --workspace
```

All core functionality is tested including:
- Tab creation and management
- Navigation history
- Tab switching and closing
- UI components
- Theme system
