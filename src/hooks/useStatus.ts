import { useCallback, useEffect, useState } from "react";
import { getStatus, type Status } from "../api";

const POLL_INTERVAL_MS = 2000;

export function useStatus() {
  const [status, setStatus] = useState<Status | null>(null);

  const refresh = useCallback(async () => {
    setStatus(await getStatus());
  }, []);

  useEffect(() => {
    let active = true;
    const poll = async () => {
      const next = await getStatus();
      if (active) {
        setStatus(next);
      }
    };
    void poll();
    const timer = window.setInterval(() => void poll(), POLL_INTERVAL_MS);
    return () => {
      active = false;
      window.clearInterval(timer);
    };
  }, []);

  return { status, refresh };
}
