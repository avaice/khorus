import { Check } from "lucide-react";
import type { ReactNode } from "react";
import styles from "./ChoiceList.module.css";

export type ChoiceItem<Id extends string> = {
  id: Id;
  label: string;
  description?: string;
  trailing?: ReactNode;
};

type ChoiceListProps<Id extends string> = {
  items: readonly ChoiceItem<Id>[];
  selectedId: Id | undefined;
  onSelect: (id: Id) => void;
  disabled?: boolean;
};

export function ChoiceList<Id extends string>({
  items,
  selectedId,
  onSelect,
  disabled = false,
}: ChoiceListProps<Id>) {
  return (
    <ul className={styles.list}>
      {items.map((item) => (
        <li key={item.id} className={styles.row}>
          <button
            type="button"
            role="radio"
            aria-checked={item.id === selectedId}
            className={styles.main}
            disabled={disabled}
            onClick={() => onSelect(item.id)}
          >
            <span className={styles.check}>
              {item.id === selectedId && <Check size={16} />}
            </span>
            <span className={styles.text}>
              <span className={styles.label}>{item.label}</span>
              {item.description && (
                <span className={styles.description}>{item.description}</span>
              )}
            </span>
          </button>
          {item.trailing}
        </li>
      ))}
    </ul>
  );
}
