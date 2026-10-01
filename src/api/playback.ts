import { invoke } from "@tauri-apps/api/core";

export const getPlayOnRepeat = () => invoke<boolean>("get_play_on_repeat");

export const setPlayOnRepeat = (enabled: boolean) =>
  invoke<void>("set_play_on_repeat", { enabled });
