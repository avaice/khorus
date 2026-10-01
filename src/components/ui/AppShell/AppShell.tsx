import type { ReactNode } from "react";
import styles from "./AppShell.module.css";

type AppShellProps = {
  header: ReactNode;
  menu: ReactNode;
  children: ReactNode;
};

export function AppShell({ header, menu, children }: AppShellProps) {
  return (
    <div className={styles.shell}>
      <header className={styles.header} data-tauri-drag-region>
        {header}
      </header>
      <nav className={styles.menu} data-tauri-drag-region>
        {menu}
      </nav>
      <main className={styles.content}>{children}</main>
    </div>
  );
}
