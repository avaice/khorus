import type { ReactNode } from "react";
import styles from "./Text.module.css";

type TextProps = {
  tone?: "muted" | "danger";
  children: ReactNode;
};

export function Text({ tone = "muted", children }: TextProps) {
  return (
    <p className={styles.text} data-tone={tone}>
      {children}
    </p>
  );
}
