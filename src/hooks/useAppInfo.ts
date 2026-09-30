import { getName, getVersion } from "@tauri-apps/api/app";
import { useEffect, useState } from "react";

type AppInfo = {
  name: string;
  version: string;
};

export function useAppInfo() {
  const [info, setInfo] = useState<AppInfo | null>(null);

  useEffect(() => {
    let active = true;
    Promise.all([getName(), getVersion()]).then(([name, version]) => {
      if (active) {
        setInfo({ name, version });
      }
    });
    return () => {
      active = false;
    };
  }, []);

  return info;
}
