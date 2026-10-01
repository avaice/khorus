import { use, useEffect, useState } from "react";
import { getKeyMap } from "../api";
import { once } from "./once";

const loadInitialKeyMap = once(() => getKeyMap().catch(() => null));

export function useKeyMap(packId: string | undefined) {
  const [keyMap, setKeyMap] = useState(use(loadInitialKeyMap()));

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
