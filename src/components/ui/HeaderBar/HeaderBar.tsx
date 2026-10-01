import type { ReactNode } from "react";
import styles from "./HeaderBar.module.css";

type HeaderBarProps = {
  title: string;
  actions?: ReactNode;
};

export function HeaderBar({ title, actions }: HeaderBarProps) {
  return (
    <div className={styles.bar} data-tauri-drag-region>
      <h1 className={styles.title}>{title}</h1>
      {actions}
    </div>
  );
}
