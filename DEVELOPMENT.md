# Developer Guide

## Prerequisites

- Node.js
- Rust (latest stable)
- Xcode Command Line Tools

## Supported Platforms

- macOS on Apple Silicon

## Commands

- Install dependencies: npm install
- Run in development mode: npm run tauri dev
- Build: npm run tauri build
- Lint: npm run lint
- Format: npm run format
- Type check: npm run typecheck
- Rust tests: cargo test in the src-tauri directory
- Rust lint and format: cargo clippy and cargo fmt in the src-tauri directory

## Project Structure

- src: Frontend (React and TypeScript)
- src-tauri: Backend (Rust and Tauri)
- src-tauri/builtin-packs: Built-in sound packs
- install.sh: Install script for users
- docs/philosophy: What the product aims to do (no technical details)
- docs/specs: Sound pack specification

## Adding a Built-in Sound Pack

1. Create a folder in src-tauri/builtin-packs and place pack.json and the required audio files in it
2. Add an entry to the list in src-tauri/src/builtin.rs
3. See docs/specs/sound-pack.md for the specification

## Updating the App Icon

1. Replace src-tauri/icons/app-icon.png (1024px square)
2. Run npx tauri icon src-tauri/icons/app-icon.png
3. Delete the generated android, ios, and 64x64.png, which are not used
4. The menu bar icon (tray.png) is a separate image, so replace it separately if needed

## Development Notes

- Enabling launch at login registers the development executable as a login item. Turn it off after testing
- Input monitoring permission is required per executable. Grant it separately for the development build and the built app
- Only include audio files in the repository whose licenses have been verified for distribution

## Releasing

1. Update the version in package.json, src-tauri/tauri.conf.json, and src-tauri/Cargo.toml
2. Create and publish a Release on GitHub
3. GitHub Actions builds the app and attaches Khorus.zip to the Release

- install.sh downloads Khorus.zip from the latest Release
- The app is only ad-hoc signed and is not notarized
