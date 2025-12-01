# Horizon Browser UI Guide

## Visual Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  [ Tab 1: about:home ]  [ Tab 2: example... ]  [ ✕ ]  [ ➕ ]        │ ← Tab Bar
├─────────────────────────────────────────────────────────────────────┤
│  [ ◀ ] [ ▶ ] [ ⟳ ]  [  about:home                           ] [ Go ]│ ← Navigation Bar
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│                                                                       │
│                         🌅 Horizon Browser                           │
│                                                                       │
│                A Lightweight, Privacy-Focused Browser                │
│                                                                       │
│                                                                       │
│           🔒                    ⚡                    🎨             │
│       Privacy First        Fast & Lightweight     Modern UI          │
│     Built-in tracking      Minimal resource      Clean dark theme    │
│       protection              usage                 interface        │
│                                                                       │
│                                                                       │
│          Enter a URL in the address bar to start browsing            │
│                                                                       │
│                                                                       │ ← Content Area
└─────────────────────────────────────────────────────────────────────┘
```

## UI Components

### 1. Tab Bar (Top Section)

**Background**: Dark secondary (#161b22)
**Height**: Auto-sized based on content

#### Tab Display
- **Active Tab**: Lighter background (#0d1117), white text (#e6edf3)
- **Inactive Tabs**: Darker background (#161b22), muted text (#7d8ca0)
- **Tab Format**: `[ Title... ] [ ✕ ]`
- **Title Truncation**: Titles longer than 25 characters are truncated with "..."
- **Hover Effect**: Subtle highlight on hover
- **Border**: 1px border (#30363d) around each tab

#### New Tab Button
- **Symbol**: ➕
- **Position**: Right end of tab bar
- **Action**: Creates new tab with about:home

### 2. Navigation Bar (Below Tab Bar)

**Background**: Dark secondary (#161b22)
**Height**: Fixed height for controls

#### Components (Left to Right)

1. **Back Button** (◀)
   - Enabled when history exists
   - Disabled state: Grayed out
   - Tooltip: "Go back"

2. **Forward Button** (▶)
   - Enabled when forward history exists
   - Disabled state: Grayed out
   - Tooltip: "Go forward"

3. **Reload Button** (⟳)
   - Always enabled
   - Tooltip: "Reload page"

4. **Address Bar** (Main Input)
   - Full-width text input
   - Hint text: "Enter URL or search..."
   - Background: Slightly lighter than bar
   - Text color: Bright (#e6edf3)
   - Border: Subtle border
   - Accepts keyboard input (Enter to navigate)

5. **Go Button**
   - Text: "Go"
   - Action: Navigate to URL in address bar

### 3. Content Area (Main Window)

**Background**: Primary dark (#0d1117)
**Scroll**: Vertical scroll enabled

#### Home Page (about:home)

**Layout**: Centered vertical layout

1. **Logo Section**
   - Emoji: 🌅
   - Text: "Horizon Browser"
   - Font size: 48px
   - Color: Accent blue (#58a6ff)

2. **Tagline**
   - Text: "A Lightweight, Privacy-Focused Browser"
   - Font size: 20px
   - Color: Primary text (#e6edf3)

3. **Features Grid**
   - Three columns, horizontally arranged
   - Each feature has:
     - Emoji icon (32px)
     - Feature name (16px, bold)
     - Description (2 lines)

4. **Getting Started**
   - Small hint text at bottom
   - Color: Secondary muted (#7d8ca0)

#### Blank Page (about:blank)

**Layout**: Centered
- Single line of text: "about:blank"
- Font size: 24px
- Color: Secondary muted (#7d8ca0)

#### Web Page Preview

**Layout**: Left-aligned with padding

1. **Header**
   - Title: "Web Page Preview"
   - Font size: 28px
   - Color: Accent blue (#58a6ff)

2. **URL Display**
   - Emoji: 📄
   - Full URL
   - Font size: 16px

3. **Description**
   - Explanation text about placeholder
   - Color: Secondary muted

4. **Simulated Content Frame**
   - Background: Secondary dark (#161b22)
   - Border: 1px (#30363d)
   - Padding: 20px
   - Contains sample text and list items

## Color Palette

### Backgrounds
- **Primary**: #0d1117 (Darkest, main window)
- **Secondary**: #161b22 (Tab bar, nav bar, frames)
- **Tertiary**: #21262d (Borders, highlights)

### Text
- **Primary**: #e6edf3 (Bright white, main text)
- **Secondary**: #7d8ca0 (Muted, less important)
- **Muted**: #57606a (Subtle, hints)

### Accents
- **Primary**: #58a6ff (Blue, links, highlights)
- **Success**: #3fb950 (Green)
- **Warning**: #bb8009 (Orange)
- **Danger**: #f85149 (Red)

### Borders
- **Default**: #30363d (Visible borders)
- **Muted**: #21262d (Subtle borders)

## Interactions

### Mouse Actions

1. **Click Tab**: Switch to that tab
2. **Click ✕ on Tab**: Close that tab (if not last tab)
3. **Click ➕**: Create new tab
4. **Click ◀**: Go back in history
5. **Click ▶**: Go forward in history
6. **Click ⟳**: Reload current page
7. **Click in Address Bar**: Focus for typing
8. **Click Go**: Navigate to entered URL

### Keyboard Actions

1. **Type in Address Bar**: Enter URL or search query
2. **Press Enter**: Navigate to URL
3. **Tab Key**: Navigate between UI elements

### Hover Effects

- **Tabs**: Slightly lighter background on hover
- **Buttons**: Highlight on hover
- **Address Bar**: Focus border on interaction

## Responsive Behavior

### Window Resize
- Address bar expands/contracts with window width
- Content area maintains scroll
- Tab bar wraps if too many tabs (handled by UI framework)

### Tab Overflow
- Tabs maintain minimum size
- Title truncation prevents overflow
- Horizontal scrolling enabled if needed (framework dependent)

## Accessibility

### Visual Indicators
- Active tab clearly distinguished from inactive
- Disabled buttons clearly grayed out
- Focus states visible

### Tooltips
- Available on navigation buttons
- Helpful hover text

### Color Contrast
- High contrast between text and background
- Meets accessibility standards for dark theme

## Theme Consistency

All UI elements follow the GitHub-inspired dark theme:
- Consistent spacing (8px base unit)
- Rounded corners (4px radius)
- Subtle borders (1px width)
- Smooth transitions
- Professional, modern appearance

## User Experience

### First Launch
1. Window opens with single tab
2. Tab shows "about:home"
3. Address bar displays "about:home"
4. Welcome page loads with features

### Typical Workflow
1. User types URL in address bar
2. Presses Enter or clicks Go
3. Content updates to show page
4. User can open new tabs
5. Switch between tabs as needed
6. Navigate with back/forward
7. Close tabs when done

### Navigation Logic
- **URL Detection**: Automatically adds https:// if needed
- **Search Detection**: Non-URL text searches via DuckDuckGo
- **History Management**: Each tab maintains independent history
- **Tab Protection**: Last tab cannot be closed
