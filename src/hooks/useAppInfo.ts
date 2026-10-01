import { getName, getVersion } from "@tauri-apps/api/app";
import { useEffect, useState } from "react";

const UNRELEASED_VERSION = "0.0.0";
const DEV_VERSION_LABEL = "DEV";

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
        setInfo({
          name,
          version: version === UNRELEASED_VERSION ? DEV_VERSION_LABEL : version,
        });
      }
    });
    return () => {
      active = false;
    };
  }, []);

  return info;
}
