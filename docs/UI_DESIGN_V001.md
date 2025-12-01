# Horizon Browser UI & Design v0.0.1

## Overview

This document describes the UI & Design Overhaul for Horizon Browser v0.0.1, which introduces a modern, dark, minimalistic interface inspired by contemporary browsers like Arc and Vivaldi.

## Design Language

### Color Palette

#### Dark Theme (Default)
- **Background Gradient**: `#1E1E1E` to `#2A2A2A` (30,30,30 to 42,42,42 RGB)
- **Panel Backgrounds**: `#222222` (34,34,34 RGB)
- **Neon Blue Accent**: `#00BFFF` (0,191,255 RGB) - Used for active states, focus indicators, and primary actions
- **Neon Purple Accent**: `#A020F0` (160,32,240 RGB) - Used for highlights and secondary branding
- **Text Primary**: `#E6EDF3` (230,237,243 RGB)
- **Text Secondary**: `#7D8CA0` (125,140,160 RGB)
- **Text Muted**: `#57606A` (87,96,106 RGB)

### Shape Language

- **Border Radius**: 8px on all interactive elements (buttons, panels, cards)
- **Tab Shape**: Fully rounded pill shape with 8px radius
- **Stroke Width**: 1-2px for borders (2px for active/focused states)

### Typography

- **Headings**: 20-64px, bold weight
- **Body Text**: 14-16px, regular weight
- **Small Text**: 12-13px for hints and descriptions

## Layout Regions

### 1. Title Bar and Tab Strip

**Location**: Top of window

**Features**:
- Pill-shaped tabs with 8px rounded corners
- Active tabs: Neon blue (#00BFFF) 2px border
- Inactive tabs: Subtle gray border
- New tab button (➕) on the right
- Tab close buttons (✕) on each tab
- Loading indicators (⟳) for tabs in loading state

**Colors**:
- Background: `#222222` (34,34,34)
- Active tab: `#2A2A2A` (42,42,42) with neon blue border
- Inactive tab: `#1E1E1E` (30,30,30)

### 2. Left Sidebar Navigation

**Location**: Left side of window

**Dimensions**:
- Expanded: 250px width
- Collapsed: 60px width (icon-only mode)

**Navigation Items**:
1. 🔍 Search
2. ⭐ Bookmarks
3. 🕐 History
4. ⚙ Settings
5. 📈 Stocks
6. 🌤 Weather

**Features**:
- Toggle button (☰/◀) to collapse/expand
- Hover states with subtle background color change
- Active items: Highlighted with neon blue border and darker background
- Smooth transitions between states

**Colors**:
- Background: `#1E1E1E` (30,30,30)
- Active item: `#2A2A2A` with neon blue border
- Hover: `#2A2A2A` (42,42,42)

### 3. Address Bar

**Location**: Below tab strip, above content area

**Features**:
- Navigation controls: Back (◀), Forward (▶), Reload (⟳), Home (🏠)
- Security indicator: Lock icon (🔒) for HTTPS, warning (⚠) for HTTP, info (ℹ) for internal pages
- URL input field with glassmorphism-inspired styling
- Bookmark button (⭐)
- Profile button (👤)

**Colors**:
- Background: `#222222` (34,34,34)
- Button background: Transparent with hover state
- Focused address bar: Neon blue border

**Interactions**:
- All buttons have 6px border radius
- Keyboard shortcuts displayed in tooltips
- Enter key submits URL navigation

### 4. Home Page / Dashboard

**Location**: Central content area when navigating to `about:home`

**Layout Structure**:
1. **Header Section**
   - Large "🌅 Horizon" title in neon blue
   - Subtitle in neon purple
   - Central search bar with neon blue border

2. **App Shortcut Cards** (3-column grid)
   - Designer (🎨) - Creative tools
   - Complex Shader (✨) - GPU rendering
   - News (📰) - Latest updates
   
   Card styling:
   - Background: `#1E1E1E` (30,30,30)
   - Border: `#333333` (51,51,51)
   - 8px border radius
   - Icon (48px), Title (18px), Subtitle (13px)

3. **Widget Section** (2-column layout)
   
   **Weather Widget**:
   - Current temperature and conditions
   - Weather icon (☀️/🌤)
   - Humidity information (💧)
   - Card format with separator lines
   
   **News Feed Widget**:
   - Latest headlines with icons
   - Thumbnail placeholders (📷)
   - Title and description for each item
   - Separator lines between items

4. **Social Media Section**
   - Connect label
   - Icon buttons: Facebook (📘), Instagram (📷), Twitter (🐦)

### 5. Settings View

**Location**: Central content area when navigating to `about:settings`

**Layout Structure**:
1. **Top Banner**
   - "🌅 Horizon Settings" header in neon blue
   - Descriptive subtitle
   - Background: `#222222` (34,34,34)

2. **Left Navigation Sidebar** (220px wide)
   - Settings categories as buttons
   - Active category: Neon blue border and darker background
   - Icons with labels
   - 8px border radius on buttons

3. **Settings Panels**
   Categories:
   - 🏠 General
   - 🔒 Privacy
   - 🎨 Appearance
   - 🌐 Network & VPN
   - 🔑 Passwords
   - 🧩 Extensions
   - 📥 Downloads
   - 🔧 Advanced

4. **Content Area**
   - Card-based layout for settings groups
   - Each setting in its own card
   - Setting name (16px, bold)
   - Control (input, checkbox, combobox, slider)
   - Description text (12px, muted color)
   - Cards: `#222222` background with `#333333` border

5. **Save Button**
   - Bottom of content area
   - Large button (150px × 40px)
   - Neon blue background
   - "💾 Save Settings" label

## Responsive Design

### Window Sizing
- **Minimum recommended**: 1024×768
- **Default**: 1280×720
- **Responsive**: Layouts adapt to window width
- **High-DPI**: All UI elements scale properly

### Breakpoints
- Content areas use max-width constraints (700px for settings content)
- Widgets stack vertically on narrow screens
- Sidebar can collapse to icon-only mode for space saving

## Interaction Patterns

### Hover States
- Subtle background color change (typically to `#2A2A2A`)
- No dramatic animations, maintaining performance

### Active States
- Neon blue accent color
- 2px border for emphasis
- Slightly darker background

### Focus States
- Neon blue border on focused elements
- Clear keyboard navigation support

### Transitions
- Duration: ~200ms (not explicitly implemented in egui, but design intent)
- Easing: ease-in-out
- Applied to: color changes, background changes, border changes

## Accessibility

### Contrast Ratios
- Primary text on dark background: High contrast (>7:1)
- Secondary text: Medium contrast (>4.5:1)
- Interactive elements: Clear visual distinction

### Keyboard Navigation
- Full keyboard support via egui framework
- Tab navigation through all interactive elements
- Enter/Space for activation
- Escape for dialogs and cancellations

### Visual Feedback
- Clear hover states
- Distinct active/inactive states
- Loading indicators
- Tooltips with helpful information and shortcuts

## Technical Implementation

### Framework
- **UI Library**: egui (eframe) v0.29
- **Language**: Rust
- **Platform**: Cross-platform (Windows, macOS, Linux)

### Key Modules
- `ui/src/theme.rs` - Color palette and theme definitions
- `ui/src/sidebar.rs` - Sidebar navigation component
- `ui/src/window.rs` - Main browser window and layout
- `ui/src/tabs.rs` - Tab management
- `ui/src/settings.rs` - Settings UI and persistence

### Performance Considerations
- UI updates are event-driven (via egui's reactive model)
- Minimal allocations in hot paths
- Efficient state management using Rust's ownership system
- No unnecessary redraws

## Future Enhancements

### Planned Features
1. **Theme Switching**: Runtime toggle between dark and light themes
2. **Animations**: Smooth transitions using egui's animation system
3. **Customization**: User-adjustable colors and spacing
4. **Additional Widgets**: More dashboard widgets (stocks, crypto, calendar)
5. **Tab Features**: Drag-and-drop reordering (when egui supports it)
6. **Glassmorphism**: True backdrop blur effects (platform-dependent)

### Extensibility
- Theme system designed for easy addition of new color schemes
- Widget system allows for custom dashboard components
- Settings panels can be easily extended with new categories
- Sidebar can accommodate additional navigation items

## Design Rationale

### Why Dark Theme?
- Reduces eye strain during extended use
- Modern aesthetic preferred by developers
- Better battery life on OLED screens
- Aligns with contemporary browser trends

### Why Neon Accents?
- Provides clear visual hierarchy
- Creates distinctive brand identity
- Excellent contrast against dark backgrounds
- Modern, tech-forward appearance

### Why 8px Border Radius?
- Balance between sharp and overly rounded
- Comfortable, approachable feeling
- Standard in modern design systems
- Works well across different scales

## Screenshots

*Note: Screenshots cannot be automatically generated in a headless environment. To see the UI in action, build and run the application:*

```bash
cargo run --release
```

### Expected Views

1. **Home Page**: Clean dashboard with search bar, app cards, weather, and news widgets
2. **Settings Page**: Organized settings with left navigation and card-based controls
3. **Browsing**: Tab strip with active indicators and sidebar navigation
4. **Address Bar**: Modern controls with security indicators

## Comparison to Original Design

### Improvements over Previous Version
1. **Visual Consistency**: Unified color palette throughout
2. **Modern Aesthetic**: Gradient-inspired backgrounds and neon accents
3. **Better Information Hierarchy**: Clear visual distinction between UI elements
4. **Enhanced Usability**: Sidebar navigation and improved tab management
5. **Richer Home Page**: Dashboard with widgets instead of simple placeholder
6. **Professional Settings**: Card-based layout with clear categorization

### Maintained Features
- Fast performance
- Cross-platform compatibility
- Clean, minimal design
- Privacy-focused approach
- Keyboard shortcuts

## Resources

### Color Palette Reference
- [Color palette definition](../ui/src/theme.rs)
- Use `ColorPalette::dark()` for default theme
- All colors available through `Theme` struct

### Component Examples
- Sidebar: See `ui/src/sidebar.rs`
- Tab styling: See `ui/src/window.rs` (render tab section)
- Settings layout: See `ui/src/window.rs` (render_settings_page)
- Home page: See `ui/src/window.rs` (render_home_page)

## License

This design system is part of Horizon Browser and follows the same MIT license.
