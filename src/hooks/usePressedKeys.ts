import { useEffect, useState } from "react";
import { onKeyPress } from "../api";

const HIGHLIGHT_MS = 150;

export function usePressedKeys() {
  const [pressed, setPressed] = useState<ReadonlySet<string>>(new Set());

  useEffect(() => {
    const timers = new Map<string, number>();

    const release = (key: string) => {
      timers.delete(key);
      setPressed((current) => {
        const next = new Set(current);
        next.delete(key);
        return next;
      });
    };

    const press = (key: string) => {
      window.clearTimeout(timers.get(key));
      timers.set(
        key,
        window.setTimeout(() => release(key), HIGHLIGHT_MS),
      );
      setPressed((current) => new Set(current).add(key));
    };

    const unlisten = onKeyPress(press);
    return () => {
      void unlisten.then((stop) => stop());
      timers.forEach((timer) => window.clearTimeout(timer));
    };
  }, []);

  return pressed;
}
