import type { ReactNode } from "react";
import styles from "./IconButton.module.css";

type IconButtonProps = {
  label: string;
  icon: ReactNode;
  onClick: () => void;
};

export function IconButton({ label, icon, onClick }: IconButtonProps) {
  return (
    <button
      type="button"
      className={styles.button}
      aria-label={label}
      title={label}
      onClick={onClick}
    >
      {icon}
    </button>
  );
}
