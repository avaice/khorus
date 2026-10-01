[English](README.md) | [日本語](README.ja.md)

# Khorus

A noooisy app that makes a sound every time you hit a key

## Features

- Plays sound effects as you type
- Load sound packs to use any sounds you like
- Lives in the menu bar
- Supports launching automatically at login

## Requirements

- macOS 11 or later
- Apple Silicon
  - Intel Mac users can probably get it working by building it themselves

## Installation

`curl -fsSL https://raw.githubusercontent.com/avaice/khorus/main/install.sh | bash`

- If you'd rather not use the script, download the binary from Releases

### Uninstallation

1. Quit Khorus
2. Delete Khorus.app from /Applications
3. To also remove settings and sound packs, delete ~/Library/Application Support/com.avaice.khorus

- If you enabled automatic launch at login, turn it off under "Startup Settings" on the Home screen before uninstalling

## Usage

### Getting Started

1. Launch Khorus
2. Click "Request Permission" on the Home screen
3. In System Settings, go to "Privacy & Security" → "Input Monitoring" and allow Khorus
4. Enjoy...

## Creating Sound Packs

- Place pack.json, which defines the key-to-sound mappings, and the audio files at the root of the zip archive
- Built-in macOS sounds can be specified by name, such as macos:Tink
- See docs/specs/sound-pack.md for the full specification
  - We recommend handing it to an AI agent, which should make creating packs easy

## For Developers

- See DEVELOPMENT.md for development environment setup and build instructions
