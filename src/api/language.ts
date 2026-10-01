import { invoke } from "@tauri-apps/api/core";

export type Locale = "ja" | "en";

export type Language = "system" | Locale;

export type LanguageState = {
  language: Language;
  locale: Locale;
};

export const getLanguage = () => invoke<LanguageState>("get_language");

export const setLanguage = (language: Language) =>
  invoke<LanguageState>("set_language", { language });
