import { invoke } from "@tauri-apps/api/core";

export type PackSummary = {
  id: string;
  title: string;
  description: string;
  builtin: boolean;
  selected: boolean;
};

export const listPacks = () => invoke<PackSummary[]>("list_packs");

export const selectPack = (id: string) => invoke<void>("select_pack", { id });

export const importPack = (path: string) =>
  invoke<string>("import_pack", { path });

export const deletePack = (id: string) => invoke<void>("delete_pack", { id });
