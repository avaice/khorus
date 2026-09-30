export type MenuItem<Id extends string> = {
  id: Id;
  label: string;
};

type MenuListProps<Id extends string> = {
  items: readonly MenuItem<Id>[];
  activeId: Id;
  onSelect: (id: Id) => void;
};

export function MenuList<Id extends string>({
  items,
  activeId,
  onSelect,
}: MenuListProps<Id>) {
  return (
    <ul className="menu-list">
      {items.map((item) => (
        <li key={item.id}>
          <button
            type="button"
            className="menu-list-item"
            aria-current={item.id === activeId ? "page" : undefined}
            onClick={() => onSelect(item.id)}
          >
            {item.label}
          </button>
        </li>
      ))}
    </ul>
  );
}
