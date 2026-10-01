import { listen } from "@tauri-apps/api/event";

export const onKeyPress = (handler: (key: string) => void) =>
  listen<string>("key-press", (event) => handler(event.payload));
