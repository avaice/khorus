import { Check } from "lucide-react";
import type { ReactNode } from "react";

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
    <ul className="choice-list">
      {items.map((item) => (
        <li key={item.id} className="choice-list-row">
          <button
            type="button"
            role="radio"
            aria-checked={item.id === selectedId}
            className="choice-list-main"
            disabled={disabled}
            onClick={() => onSelect(item.id)}
          >
            <span className="choice-list-check">
              {item.id === selectedId && <Check size={16} />}
            </span>
            <span className="choice-list-text">
              <span className="choice-list-label">{item.label}</span>
              {item.description && (
                <span className="choice-list-description">
                  {item.description}
                </span>
              )}
            </span>
          </button>
          {item.trailing}
        </li>
      ))}
    </ul>
  );
}
