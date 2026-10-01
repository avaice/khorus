import styles from "./Slider.module.css";

type SliderProps = {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
};

export function Slider({ label, value, min, max, onChange }: SliderProps) {
  return (
    <label className={styles.slider}>
      <span>{label}</span>
      <input
        type="range"
        className={styles.input}
        min={min}
        max={max}
        step={1}
        value={value}
        onChange={(event) => onChange(event.currentTarget.valueAsNumber)}
      />
      <span className={styles.value}>{value}</span>
    </label>
  );
}
