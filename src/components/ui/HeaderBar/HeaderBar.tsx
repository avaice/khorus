import type { ReactNode } from "react";

type HeaderBarProps = {
  title: string;
  actions?: ReactNode;
};

export function HeaderBar({ title, actions }: HeaderBarProps) {
  return (
    <div className="header-bar" data-tauri-drag-region>
      <h1 className="header-bar-title">{title}</h1>
      {actions}
    </div>
  );
}
