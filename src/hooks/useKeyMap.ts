import { useEffect, useState } from "react";
import { getKeyMap, type KeyMap } from "../api";

export function useKeyMap(packId: string | undefined) {
  const [keyMap, setKeyMap] = useState<KeyMap | null>(null);

  useEffect(() => {
    if (!packId) {
      return;
    }
    let active = true;
    getKeyMap().then((loaded) => {
      if (active) {
        setKeyMap(loaded);
      }
    });
    return () => {
      active = false;
    };
  }, [packId]);

  return keyMap;
}
