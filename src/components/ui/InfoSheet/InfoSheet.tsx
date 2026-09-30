import type { ReactNode } from "react";

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
    <section className="info-sheet">
      <h2 className="info-sheet-title">{title}</h2>
      <dl className="info-sheet-list">
        {items.map((item) => (
          <div key={item.label} className="info-sheet-row">
            <dt className="info-sheet-label">{item.label}</dt>
            <dd className="info-sheet-value">{item.value}</dd>
          </div>
        ))}
      </dl>
    </section>
  );
}
