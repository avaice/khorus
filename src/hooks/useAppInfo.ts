import { getName, getVersion } from "@tauri-apps/api/app";
import { use } from "react";
import { once } from "./once";

const UNRELEASED_VERSION = "0.0.0";
const DEV_VERSION_LABEL = "DEV";

const loadAppInfo = once(async () => {
  const [name, version] = await Promise.all([getName(), getVersion()]);
  return {
    name,
    version: version === UNRELEASED_VERSION ? DEV_VERSION_LABEL : version,
  };
});

export function useAppInfo() {
  return use(loadAppInfo());
}
