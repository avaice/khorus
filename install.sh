#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Khorus"
DEFAULT_DOWNLOAD_URL="https://github.com/avaice/khorus/releases/latest/download/Khorus.zip"
DOWNLOAD_URL="${KHORUS_DOWNLOAD_URL:-$DEFAULT_DOWNLOAD_URL}"
INSTALL_DIR="${KHORUS_INSTALL_DIR:-/Applications}"

fail() {
  echo "エラー: $1" >&2
  exit 1
}

if [ "$(uname -s)" != "Darwin" ]; then
  fail "${APP_NAME} は macOS でのみ使えます"
fi

if [ "$(uname -m)" != "arm64" ]; then
  fail "${APP_NAME} は Apple シリコンの Mac でのみ使えます"
fi

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

echo "${APP_NAME} をダウンロードしています..."
curl -fL --progress-bar "$DOWNLOAD_URL" -o "$work_dir/${APP_NAME}.zip"

echo "展開しています..."
ditto -x -k "$work_dir/${APP_NAME}.zip" "$work_dir/extracted"
source_app="$work_dir/extracted/${APP_NAME}.app"
[ -d "$source_app" ] || fail "${APP_NAME}.app がダウンロードしたファイルに含まれていません"

if pgrep -x "$APP_NAME" >/dev/null 2>&1; then
  echo "起動中の ${APP_NAME} を終了します..."
  pkill -x "$APP_NAME" || true
  sleep 1
fi

destination="$INSTALL_DIR/${APP_NAME}.app"
echo "${destination} にインストールします..."
if [ -w "$INSTALL_DIR" ]; then
  rm -rf "$destination"
  ditto "$source_app" "$destination"
else
  echo "管理者権限が必要です。パスワードを入力してください。"
  sudo rm -rf "$destination"
  sudo ditto "$source_app" "$destination"
fi

xattr -dr com.apple.quarantine "$destination" 2>/dev/null || true

echo "インストールが完了しました。${APP_NAME} を起動します。"
open "$destination"
