import { useCallback, useEffect, useState } from "react";
import { listPacks, type PackSummary } from "../api";

export function usePacks() {
  const [packs, setPacks] = useState<PackSummary[]>([]);

  const refresh = useCallback(async () => {
    setPacks(await listPacks());
  }, []);

  useEffect(() => {
    let active = true;
    listPacks().then((next) => {
      if (active) {
        setPacks(next);
      }
    });
    return () => {
      active = false;
    };
  }, []);

  return { packs, refresh };
}
