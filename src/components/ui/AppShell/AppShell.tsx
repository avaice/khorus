import type { ReactNode } from "react";

type AppShellProps = {
  header: ReactNode;
  menu: ReactNode;
  children: ReactNode;
};

export function AppShell({ header, menu, children }: AppShellProps) {
  return (
    <div className="app-shell">
      <header className="app-shell-header" data-tauri-drag-region>
        {header}
      </header>
      <nav className="app-shell-menu" data-tauri-drag-region>
        {menu}
      </nav>
      <main className="app-shell-content">{children}</main>
    </div>
  );
}
