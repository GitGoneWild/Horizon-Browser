# Horizon Browser Settings Guide

This document provides a comprehensive guide to all available settings in Horizon Browser.

## Accessing Settings

You can access the settings page in several ways:

1. Click the **⚙ Settings** button in the navigation bar
2. Navigate to `about:settings` in the address bar
3. The settings page will open in a new tab

## Settings Panels

### General Settings

Configure basic browser behavior and preferences.

#### Homepage
- **Description**: The URL that opens when you start the browser or click the Home button
- **Default**: `about:home`
- **Customization**: Enter any valid URL or use special URLs like `about:blank`

#### Default Search Engine
- **Options**: DuckDuckGo, Google, Bing, Brave
- **Default**: DuckDuckGo
- **Description**: The search engine used when you enter search terms (not URLs) in the address bar
- **Privacy Note**: DuckDuckGo is selected by default for better privacy

#### Restore Tabs on Startup
- **Description**: Automatically restore your previous browsing session when you start the browser
- **Default**: Disabled
- **Note**: When enabled, all tabs from your last session will be reopened

### Privacy & Security Settings

Protect your privacy and enhance security while browsing.

#### Enable Tracking Protection
- **Default**: Enabled ✅
- **Description**: Blocks known trackers from loading on websites
- **Benefits**: 
  - Reduces tracking by advertisers
  - Improves page load times
  - Protects your browsing privacy

#### Send Do Not Track
- **Default**: Enabled ✅
- **Description**: Sends a Do Not Track (DNT) header with all requests
- **Benefits**: Tells websites you don't want to be tracked
- **Note**: Not all websites respect this signal

#### Block Third-Party Cookies
- **Default**: Enabled ✅
- **Description**: Prevents websites from setting cookies from third-party domains
- **Benefits**: Prevents cross-site tracking
- **Impact**: May affect some login systems and embedded content

#### Enable HTTPS-Only Mode
- **Default**: Disabled
- **Description**: Only allows connections to secure HTTPS websites
- **Benefits**: Ensures all connections are encrypted
- **Note**: May prevent access to some older websites

#### Clear Browsing Data on Exit
- **Default**: Disabled
- **Description**: Automatically clears cookies, cache, and history when closing the browser
- **Benefits**: Maximum privacy - no traces left behind
- **Warning**: You'll lose all saved logins and preferences

### Appearance Settings

Customize the look and feel of the browser.

#### Theme
- **Options**: Dark, Light
- **Default**: Dark
- **Description**: Changes the overall color scheme of the browser
- **Current Support**: Dark theme fully implemented

#### Font Size
- **Range**: 10-20 pixels
- **Default**: 14 pixels
- **Description**: Adjusts the base font size for UI elements
- **Note**: This affects browser UI, not web content

#### Show Bookmarks Bar
- **Default**: Disabled
- **Description**: Displays a bookmarks toolbar below the address bar
- **Status**: Coming soon

### Downloads Settings

Configure how files are downloaded.

#### Download Directory
- **Default**: System downloads folder
  - Windows: `C:\Users\<username>\Downloads`
  - macOS: `/Users/<username>/Downloads`
  - Linux: `/home/<username>/Downloads`
- **Description**: The default location where downloaded files are saved

#### Always Ask Where to Save Files
- **Default**: Enabled ✅
- **Description**: Shows a file picker dialog for each download
- **Benefits**: Choose save location for each file
- **Alternative**: Disable to automatically save to download directory

### Advanced Settings

Configure technical and experimental features.

#### Enable Developer Tools
- **Default**: Disabled
- **Description**: Enables debugging and inspection features
- **Use Cases**: 
  - Web development
  - Troubleshooting issues
  - Inspecting page structure

#### Use Hardware Acceleration
- **Default**: Enabled ✅
- **Description**: Uses your GPU to accelerate rendering
- **Benefits**: Improved performance and smoother animations
- **Note**: Disable if you experience graphics issues

#### Enable Experimental Features
- **Default**: Disabled
- **Description**: Enables features that are still in development
- **Warning**: May be unstable or incomplete
- **Use Case**: Testing new features before official release

## Saving Settings

After making changes:

1. Click the **💾 Save Settings** button at the bottom of any settings panel
2. Settings are persisted to disk and will be restored on next launch
3. Changes take effect immediately

## Keyboard Shortcuts

Access settings and navigate faster:

- **Settings Navigation**: Use mouse to switch between panels
- **Address Bar**: Type `about:settings` and press Enter
- **Navigation**: Use browser shortcuts to go back/forward

## Privacy Recommendations

For maximum privacy, we recommend:

1. ✅ Enable Tracking Protection
2. ✅ Enable Do Not Track
3. ✅ Block Third-Party Cookies
4. ✅ Use DuckDuckGo as default search engine
5. ⚠️ Consider enabling HTTPS-Only Mode
6. ⚠️ Consider enabling Clear Data on Exit

## Performance Recommendations

For best performance:

1. ✅ Keep Hardware Acceleration enabled
2. ✅ Use default font size (14px)
3. ⚠️ Disable Developer Tools when not in use
4. ⚠️ Avoid enabling Clear Data on Exit (requires rebuilding cache)

## Troubleshooting

### Settings Not Saving
- Ensure you clicked the Save Settings button
- Check that the application has write permissions to the data directory
- Check logs for error messages

### Settings Reset to Default
- This may happen if the settings file is corrupted
- Delete `settings.toml` from the data directory to start fresh
- Data directory locations:
  - Windows: `%APPDATA%\Horizon`
  - macOS: `~/Library/Application Support/Horizon`
  - Linux: `~/.local/share/Horizon`

### Performance Issues
- Try disabling Hardware Acceleration
- Reduce font size
- Ensure Experimental Features are disabled

## Future Enhancements

Coming soon to settings:

- 🔜 Content filtering options
- 🔜 Extension management
- 🔜 Keyboard shortcut customization
- 🔜 Import/export settings
- 🔜 Multiple profiles
- 🔜 Sync settings across devices

## Related Documentation

- [Keyboard Shortcuts](SHORTCUTS.md)
- [Privacy Guide](PRIVACY.md)
- [Development Guide](DEVELOPMENT.md)

---

For issues or feature requests, please visit our [GitHub repository](https://github.com/GitGoneWild/Horizon-Browser).
