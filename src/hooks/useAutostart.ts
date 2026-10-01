import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { use, useCallback, useState } from "react";
import { once } from "./once";

const loadInitialEnabled = once(isEnabled);

export function useAutostart() {
  const [enabled, setEnabled] = useState(use(loadInitialEnabled()));

  const update = useCallback(async (next: boolean) => {
    if (next) {
      await enable();
    } else {
      await disable();
    }
    setEnabled(await isEnabled());
  }, []);

  return { enabled, update };
}
