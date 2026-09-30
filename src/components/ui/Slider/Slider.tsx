type SliderProps = {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
};

export function Slider({ label, value, min, max, onChange }: SliderProps) {
  return (
    <label className="slider">
      <span className="slider-label">{label}</span>
      <input
        type="range"
        className="slider-input"
        min={min}
        max={max}
        step={1}
        value={value}
        onChange={(event) => onChange(event.currentTarget.valueAsNumber)}
      />
      <span className="slider-value">{value}</span>
    </label>
  );
}
