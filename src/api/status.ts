import { invoke } from "@tauri-apps/api/core";

export type PackInfo = {
  title: string;
  description: string;
};

export type Status = {
  enabled: boolean;
  permissionGranted: boolean;
  pack: PackInfo;
};

export const getStatus = () => invoke<Status>("get_status");

export const setEnabled = (enabled: boolean) =>
  invoke<void>("set_enabled", { enabled });

export const requestPermission = () => invoke<boolean>("request_permission");
