import type { Messages } from "./messages";

export const en: Messages = {
  menu: {
    home: "Home",
    soundPack: "Sound Packs",
    about: "About",
  },
  soundEnabled: "Play Sounds",
  home: {
    permissionTitle: "Input Monitoring",
    permissionGranted: "Permission granted.",
    permissionRequiredTitle: "Input Monitoring Permission Required",
    permissionRequiredDescription:
      "To detect key presses, allow “Input Monitoring” in System Settings under “Privacy & Security”. Once allowed, it will be enabled automatically within a few seconds.",
    requestPermission: "Request Permission",
    volumeTitle: "Volume",
    enterKey: "Enter Key",
    spaceKey: "Space Key",
    otherKeys: "Other Keys",
    keyRepeatTitle: "Key Repeat",
    playOnRepeat: "Play Sounds While a Key Is Held",
    startupTitle: "Startup",
    launchAtLogin: "Launch at Login",
    languageTitle: "Language",
    systemLanguage: "Use System Setting",
  },
  soundPack: {
    title: "Sound Packs",
    import: "Import…",
    fileFilterName: "Sound Pack",
    delete: "Delete",
    deleteDialogTitle: "Delete Sound Pack",
    deleteConfirm: (title) => `Delete “${title}”?`,
  },
  about: {
    version: "Version",
  },
};
