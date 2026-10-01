export type KeyboardKeySize = "normal" | "wide" | "space";

export type KeyboardKey<Id extends string> = {
  id: Id;
  label: string;
  size?: KeyboardKeySize;
  muted?: boolean;
};

type KeyboardProps<Id extends string> = {
  rows: readonly (readonly KeyboardKey<Id>[])[];
  activeIds: ReadonlySet<string>;
  onPress: (id: Id) => void;
  onHover: (id: Id | null) => void;
};

export function Keyboard<Id extends string>({
  rows,
  activeIds,
  onPress,
  onHover,
}: KeyboardProps<Id>) {
  return (
    <div className="keyboard" onMouseLeave={() => onHover(null)}>
      {rows.map((row, index) => (
        <div key={index} className="keyboard-row">
          {row.map((key) => (
            <button
              key={key.id}
              type="button"
              className="keyboard-key"
              data-size={key.size ?? "normal"}
              data-muted={key.muted ?? false}
              data-active={activeIds.has(key.id)}
              onClick={() => onPress(key.id)}
              onMouseEnter={() => onHover(key.id)}
              onFocus={() => onHover(key.id)}
              onBlur={() => onHover(null)}
            >
              {key.label}
            </button>
          ))}
        </div>
      ))}
    </div>
  );
}
