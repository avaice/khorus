import type { Messages } from "./messages";

export const ja: Messages = {
  menu: {
    home: "ホーム",
    soundPack: "サウンドパック",
    about: "このアプリについて",
  },
  soundEnabled: "音を鳴らす",
  home: {
    permissionTitle: "入力監視",
    permissionGranted: "許可されています。",
    permissionRequiredTitle: "入力監視の許可が必要です",
    permissionRequiredDescription:
      "キー入力を検知するために、システム設定の「プライバシーとセキュリティ」から「入力監視」を許可してください。許可すると、数秒後に自動で有効になります。",
    requestPermission: "許可をリクエスト",
    volumeTitle: "音量",
    enterKey: "Enterキー",
    spaceKey: "Spaceキー",
    otherKeys: "その他のキー",
    keyRepeatTitle: "キーリピート",
    playOnRepeat: "キーを押し続けたときも鳴らす",
    startupTitle: "スタートアップ設定",
    launchAtLogin: "ログイン時に起動",
    languageTitle: "表示言語",
    systemLanguage: "システム設定に合わせる",
  },
  soundPack: {
    title: "サウンドパック",
    import: "読み込む…",
    fileFilterName: "サウンドパック",
    delete: "削除",
    deleteDialogTitle: "サウンドパックの削除",
    deleteConfirm: (title) => `「${title}」を削除しますか？`,
    keyMapTitle: "キーの割り当て",
    keyMapHint:
      "キーにカーソルを合わせると割り当てられた音を表示し、クリックすると再生します。",
    randomSound: (count) => `ランダム（${count}種類）`,
    noSound: "割り当てなし",
  },
  about: {
    version: "バージョン",
  },
};
