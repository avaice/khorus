import { openUrl } from "@tauri-apps/plugin-opener";
import type { ReactNode } from "react";

type LinkProps = {
  href: string;
  children: ReactNode;
};

export function Link({ href, children }: LinkProps) {
  return (
    <a
      className="link"
      href={href}
      onClick={(event) => {
        event.preventDefault();
        void openUrl(href);
      }}
    >
      {children}
    </a>
  );
}
