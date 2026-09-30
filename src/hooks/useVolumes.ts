import { useCallback, useEffect, useState } from "react";
import { getVolumes, setVolumes, type Volumes } from "../api";

export function useVolumes() {
  const [volumes, setLocalVolumes] = useState<Volumes | null>(null);

  useEffect(() => {
    let active = true;
    getVolumes().then((loaded) => {
      if (active) {
        setLocalVolumes(loaded);
      }
    });
    return () => {
      active = false;
    };
  }, []);

  const update = useCallback(
    (key: keyof Volumes, value: number) => {
      if (!volumes) {
        return;
      }
      const next = { ...volumes, [key]: value };
      setLocalVolumes(next);
      void setVolumes(next);
    },
    [volumes],
  );

  return { volumes, update };
}
