import { use, useCallback, useState } from "react";
import { getVolumes, setVolumes, type Volumes } from "../api";
import { once } from "./once";

const loadInitialVolumes = once(getVolumes);

export function useVolumes() {
  const [volumes, setLocalVolumes] = useState(use(loadInitialVolumes()));

  const update = useCallback(
    (key: keyof Volumes, value: number) => {
      const next = { ...volumes, [key]: value };
      setLocalVolumes(next);
      void setVolumes(next);
    },
    [volumes],
  );

  return { volumes, update };
}
