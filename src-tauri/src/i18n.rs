use std::sync::{PoisonError, RwLock};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    Ja,
    En,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    System,
    Ja,
    En,
}

impl Language {
    pub fn resolve(self) -> Locale {
        match self {
            Self::System => detect_system_locale(),
            Self::Ja => Locale::Ja,
            Self::En => Locale::En,
        }
    }
}

fn detect_system_locale() -> Locale {
    match sys_locale::get_locale() {
        Some(tag) if tag.starts_with("ja") => Locale::Ja,
        _ => Locale::En,
    }
}

pub struct Messages {
    pub sound_enabled: &'static str,
    pub open_app: &'static str,
    pub quit_app: &'static str,
    pub invalid_manifest: &'static str,
    pub unknown_key: &'static str,
    pub invalid_path: &'static str,
    pub missing_file: &'static str,
    pub too_large: &'static str,
    pub too_long: &'static str,
    pub undecodable: &'static str,
    pub unreadable_file: &'static str,
    pub invalid_zip: &'static str,
    pub too_many_files: &'static str,
    pub symlink: &'static str,
    pub pack_not_found: &'static str,
    pub builtin_locked: &'static str,
    pub file_operation_failed: &'static str,
}

const JA: Messages = Messages {
    sound_enabled: "音を鳴らす",
    open_app: "Khorusを開く",
    quit_app: "Khorusを終了",
    invalid_manifest: "pack.json を読み込めません",
    unknown_key: "不明なキー名です",
    invalid_path: "不正なパスです",
    missing_file: "ファイルが見つかりません",
    too_large: "ファイルサイズが上限を超えています",
    too_long: "音が長すぎます",
    undecodable: "音声として読み込めません",
    unreadable_file: "ファイルを読み込めません",
    invalid_zip: "zipとして読み込めません",
    too_many_files: "ファイルの数が多すぎます",
    symlink: "シンボリックリンクは使えません",
    pack_not_found: "サウンドパックが見つかりません",
    builtin_locked: "組み込みのサウンドパックは削除できません",
    file_operation_failed: "ファイルを操作できません",
};

const EN: Messages = Messages {
    sound_enabled: "Play Sounds",
    open_app: "Open Khorus",
    quit_app: "Quit Khorus",
    invalid_manifest: "Cannot read pack.json",
    unknown_key: "Unknown key name",
    invalid_path: "Invalid path",
    missing_file: "File not found",
    too_large: "File size exceeds the limit",
    too_long: "Sound is too long",
    undecodable: "Cannot decode as audio",
    unreadable_file: "Cannot read file",
    invalid_zip: "Cannot read as zip",
    too_many_files: "Too many files",
    symlink: "Symbolic links are not allowed",
    pack_not_found: "Sound pack not found",
    builtin_locked: "Built-in sound packs cannot be deleted",
    file_operation_failed: "File operation failed",
};

static CURRENT_LOCALE: RwLock<Locale> = RwLock::new(Locale::En);

pub fn locale() -> Locale {
    *CURRENT_LOCALE
        .read()
        .unwrap_or_else(PoisonError::into_inner)
}

pub fn set_locale(locale: Locale) {
    *CURRENT_LOCALE
        .write()
        .unwrap_or_else(PoisonError::into_inner) = locale;
}

pub fn messages() -> &'static Messages {
    match locale() {
        Locale::Ja => &JA,
        Locale::En => &EN,
    }
}
