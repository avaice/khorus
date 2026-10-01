import { useEffect } from "react";
import { getAccentColor } from "../api";

const ACCENT_PROPERTY = "--accent";

export function useSystemAccentColor() {
  useEffect(() => {
    let active = true;
    const apply = async () => {
      const color = await getAccentColor();
      if (active && color) {
        document.documentElement.style.setProperty(ACCENT_PROPERTY, color);
      }
    };
    void apply();
    window.addEventListener("focus", apply);
    return () => {
      active = false;
      window.removeEventListener("focus", apply);
    };
  }, []);
}
