export type Messages = {
  menu: {
    home: string;
    soundPack: string;
    about: string;
  };
  soundEnabled: string;
  home: {
    permissionTitle: string;
    permissionGranted: string;
    permissionRequiredTitle: string;
    permissionRequiredDescription: string;
    requestPermission: string;
    volumeTitle: string;
    enterKey: string;
    spaceKey: string;
    otherKeys: string;
    keyRepeatTitle: string;
    playOnRepeat: string;
    startupTitle: string;
    launchAtLogin: string;
    languageTitle: string;
    systemLanguage: string;
  };
  soundPack: {
    title: string;
    import: string;
    fileFilterName: string;
    delete: string;
    deleteDialogTitle: string;
    deleteConfirm: (title: string) => string;
  };
  about: {
    version: string;
  };
};
