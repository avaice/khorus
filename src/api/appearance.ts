import { invoke } from "@tauri-apps/api/core";

export const getAccentColor = () => invoke<string | null>("get_accent_color");
