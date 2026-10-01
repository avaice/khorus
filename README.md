# Khorus

A macOS app that plays a sound with every keystroke. Load sound packs to customize the sound of your typing.

## Features

- Play sound effects as you type
- Adjust the volume separately for Enter, Space, and other keys
- Import, switch, and delete sound packs
- Keep running in the menu bar and playing sounds after closing the window
- Launch automatically at login

## Requirements

- macOS
- A Mac with Apple silicon (Intel Macs are not supported)

## Installation

1. Open Terminal
2. Run the following command: `curl -fsSL https://raw.githubusercontent.com/avaice/khorus/main/install.sh | bash`
3. Enter your password if prompted for administrator privileges

- The app is installed in /Applications. If already installed, it will be overwritten with the update

### Uninstallation

1. Select "Quit Khorus" from the menu bar icon
2. Delete Khorus.app from /Applications
3. To also remove settings and sound packs, delete ~/Library/Application Support/com.avaice.khorus

- If you enabled automatic launch at login, turn it off under "Startup Settings" on the Home screen before uninstalling

## Usage

### Getting Started

1. Launch Khorus
2. Click "Request Permission" on the Home screen
3. In System Settings, go to "Privacy & Security" → "Input Monitoring" and allow Khorus

- Sounds will start playing automatically a few seconds after permission is granted
- Your keystrokes are never stored or transmitted. They are used only to play sounds

### Screens

- Home
  - Check input monitoring permission status
  - Adjust the volume for Enter, Space, and other keys from 0 to 10
  - Choose whether to launch at login
- Sound Packs
  - Select a sound pack from the list
  - Click "Import…" to add a sound pack in zip format
  - Delete imported sound packs using the trash button
- About
  - View the app version and other information
- Use "Play Sounds" on the right side of the header to toggle sound effects on or off

### Menu Bar

- Khorus stays in the menu bar and continues running after you close the window
- Use the menu bar icon to toggle sounds, open the window, or quit the app

## Creating Sound Packs

- Place pack.json, which defines the key-to-sound mappings, and the audio files at the root of the zip archive
- Built-in macOS sounds can be specified by name, such as macos:Tink
- See docs/specs/sound-pack.md for the full specification

## For Developers

- See DEVELOPMENT.md for development environment setup and build instructions
