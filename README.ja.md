[English](README.md) | [日本語](README.ja.md)

# Khorus

打鍵に合わせて音が鳴る、うるさ〜いアプリ

```sh
curl -fsSL https://raw.githubusercontent.com/avaice/khorus/main/install.sh | bash
```

## 特徴

- キー入力に合わせて効果音を鳴らせます
- サウンドパックを読み込めば、好きな効果音を設定可能です
- メニューバーに常駐させられます
- ログイン時の自動起動にも対応しています

## 動作環境

- macOS 11 or later
- Apple Silicon
  - Intel Macユーザーは自分でビルドすれば動くと思います

## インストール

- ページ冒頭のコマンドを実行してください
- スクリプトを使いたくない場合は、Releasesからバイナリをダウンロードしてください

### アンインストール

1. Khorusを終了する
2. /Applications の Khorus.app を削除する
3. 設定やサウンドパックも消す場合は、~/Library/Application Support/com.avaice.khorus を削除する

- 自動起動をオンにしていた場合は、アンインストールの前に、ホームの「スタートアップ設定」でオフにしてください

## 使い方

### はじめに

1. Khorus を起動する
2. ホームの「許可をリクエスト」を押す
3. システム設定の「プライバシーとセキュリティ」にある「入力監視」で、Khorus を許可する
4. enjoy...

## サウンドパックを作る

- zip の直下に、キーと音の対応を書いた pack.json と、音声ファイルを入れます
- OS 標準の音は、macos:Tink のように名前で指定できます
- 詳しい仕様は、docs/specs/sound-pack.md を参照してください
  - AI Agentに読ませれば簡単に作れると思うので、おすすめします

## 開発者の方へ

- 開発環境の準備やビルドの方法は、DEVELOPMENT.md を参照してください
