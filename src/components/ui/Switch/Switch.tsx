import styles from "./Switch.module.css";

type SwitchProps = {
  checked: boolean;
  label: string;
  onChange: (checked: boolean) => void;
};

export function Switch({ checked, label, onChange }: SwitchProps) {
  return (
    <label className={styles.switch}>
      <input
        type="checkbox"
        role="switch"
        className={styles.input}
        checked={checked}
        onChange={(event) => onChange(event.currentTarget.checked)}
      />
      <span className={styles.track} aria-hidden="true" />
      <span>{label}</span>
    </label>
  );
}
