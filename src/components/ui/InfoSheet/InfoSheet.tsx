import type { ReactNode } from "react";
import styles from "./InfoSheet.module.css";

export type InfoSheetItem = {
  label: string;
  value: ReactNode;
};

type InfoSheetProps = {
  title: string;
  items: readonly InfoSheetItem[];
};

export function InfoSheet({ title, items }: InfoSheetProps) {
  return (
    <section className={styles.sheet}>
      <h2 className={styles.title}>{title}</h2>
      <dl className={styles.list}>
        {items.map((item) => (
          <div key={item.label} className={styles.row}>
            <dt className={styles.label}>{item.label}</dt>
            <dd className={styles.value}>{item.value}</dd>
          </div>
        ))}
      </dl>
    </section>
  );
}
