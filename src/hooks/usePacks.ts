import { use, useCallback, useState } from "react";
import { listPacks } from "../api";
import { once } from "./once";

const loadInitialPacks = once(listPacks);

export function usePacks() {
  const [packs, setPacks] = useState(use(loadInitialPacks()));

  const refresh = useCallback(async () => {
    setPacks(await listPacks());
  }, []);

  return { packs, refresh };
}
