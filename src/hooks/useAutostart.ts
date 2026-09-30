import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useCallback, useEffect, useState } from "react";

export function useAutostart() {
  const [enabled, setEnabled] = useState<boolean | null>(null);

  useEffect(() => {
    let active = true;
    isEnabled().then((current) => {
      if (active) {
        setEnabled(current);
      }
    });
    return () => {
      active = false;
    };
  }, []);

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
