# 開発者向けガイド

## 必要なもの

- Node.js
- Rust(最新の安定版)
- Xcode Command Line Tools

## 対応プラットフォーム

- macOS の Apple シリコンのみ
- Intel Mac 向けのビルドと配布は行いません

## コマンド

- 依存関係のインストール: npm install
- 開発モードで起動: npm run tauri dev
- ビルド: npm run tauri build
- lint: npm run lint
- format: npm run format
- 型チェック: npm run typecheck
- Rust のテスト: src-tauri ディレクトリで cargo test
- Rust の lint と format: src-tauri ディレクトリで cargo clippy と cargo fmt

## 構成

- src: フロントエンド(React と TypeScript)
- src-tauri: バックエンド(Rust と Tauri)
- src-tauri/builtin-packs: 組み込みのサウンドパック
- install.sh: 利用者向けのインストールスクリプト
- docs/philosophy: プロダクトのやりたいこと(技術を含まない)
- docs/specs: サウンドパックの仕様

## 組み込みサウンドパックの追加

1. src-tauri/builtin-packs にフォルダを作り、pack.json と必要な音声ファイルを置く
2. src-tauri/src/builtin.rs の一覧に1件追加する
3. 仕様は docs/specs/sound-pack.md を参照する

## アプリアイコンの更新

1. src-tauri/icons/app-icon.png(1024px の正方形)を差し替える
2. npx tauri icon src-tauri/icons/app-icon.png を実行する
3. 生成された android、ios、64x64.png は使わないので削除する
4. メニューバーのアイコン(tray.png)は別の画像なので、必要なら別に差し替える

## 開発時の注意

- 自動起動をオンにすると、開発用の実行ファイルがログイン項目に登録されます。動作確認のあとはオフに戻してください
- 入力監視の許可は、実行ファイルごとに必要です。開発用とビルドしたアプリでは別々に許可してください
- 配布用の音声ファイルは、ライセンスを確認したものだけをリポジトリに含めてください

## インストールスクリプトの配布準備

- install.sh の配布 URL(DEFAULT_DOWNLOAD_URL)は未定(TBD)です
- 配布物は、Khorus.app を含む zip を想定しています
- README.md のインストール手順の URL も、決まり次第更新してください
