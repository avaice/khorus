import { invoke } from "@tauri-apps/api/core";

export type Volumes = {
  enter: number;
  space: number;
  other: number;
};

export const getVolumes = () => invoke<Volumes>("get_volumes");

export const setVolumes = (volumes: Volumes) =>
  invoke<void>("set_volumes", { volumes });
