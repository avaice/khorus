#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Khorus"
DEFAULT_DOWNLOAD_URL="https://github.com/avaice/khorus/releases/latest/download/Khorus.zip"
DOWNLOAD_URL="${KHORUS_DOWNLOAD_URL:-$DEFAULT_DOWNLOAD_URL}"
INSTALL_DIR="${KHORUS_INSTALL_DIR:-/Applications}"

fail() {
  echo "Error: $1" >&2
  exit 1
}

if [ "$(uname -s)" != "Darwin" ]; then
  fail "${APP_NAME} only runs on macOS"
fi

if [ "$(uname -m)" != "arm64" ]; then
  fail "${APP_NAME} only runs on Apple silicon Macs"
fi

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

echo "Downloading ${APP_NAME}..."
curl -fL --progress-bar "$DOWNLOAD_URL" -o "$work_dir/${APP_NAME}.zip"

echo "Extracting..."
ditto -x -k "$work_dir/${APP_NAME}.zip" "$work_dir/extracted"
source_app="$work_dir/extracted/${APP_NAME}.app"
[ -d "$source_app" ] || fail "${APP_NAME}.app was not found in the downloaded file"

if pgrep -x "$APP_NAME" >/dev/null 2>&1; then
  echo "Quitting running ${APP_NAME}..."
  pkill -x "$APP_NAME" || true
  sleep 1
fi

destination="$INSTALL_DIR/${APP_NAME}.app"
echo "Installing to ${destination}..."
if [ -w "$INSTALL_DIR" ]; then
  rm -rf "$destination"
  ditto "$source_app" "$destination"
else
  echo "Administrator privileges are required. Please enter your password."
  sudo rm -rf "$destination"
  sudo ditto "$source_app" "$destination"
fi

xattr -dr com.apple.quarantine "$destination" 2>/dev/null || true

echo "Installation complete."

printf '\033[1mLaunch %s now? [Y/n]\033[0m ' "$APP_NAME"
read -r answer 2>/dev/null </dev/tty || answer=""
case "$answer" in
  [nN]*) ;;
  *) open "$destination" ;;
esac
