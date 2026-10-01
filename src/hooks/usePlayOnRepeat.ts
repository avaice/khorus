import { useCallback, useEffect, useState } from "react";
import { getPlayOnRepeat, setPlayOnRepeat } from "../api";

export function usePlayOnRepeat() {
  const [enabled, setEnabled] = useState<boolean | null>(null);

  useEffect(() => {
    let active = true;
    getPlayOnRepeat().then((current) => {
      if (active) {
        setEnabled(current);
      }
    });
    return () => {
      active = false;
    };
  }, []);

  const update = useCallback(async (next: boolean) => {
    setEnabled(next);
    await setPlayOnRepeat(next);
  }, []);

  return { enabled, update };
}
