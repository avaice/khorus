import { use, useCallback, useState } from "react";
import { getPlayOnRepeat, setPlayOnRepeat } from "../api";
import { once } from "./once";

const loadInitialEnabled = once(getPlayOnRepeat);

export function usePlayOnRepeat() {
  const [enabled, setEnabled] = useState(use(loadInitialEnabled()));

  const update = useCallback(async (next: boolean) => {
    setEnabled(next);
    await setPlayOnRepeat(next);
  }, []);

  return { enabled, update };
}
