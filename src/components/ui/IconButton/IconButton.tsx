import type { ReactNode } from "react";

type IconButtonProps = {
  label: string;
  icon: ReactNode;
  onClick: () => void;
};

export function IconButton({ label, icon, onClick }: IconButtonProps) {
  return (
    <button
      type="button"
      className="icon-button"
      aria-label={label}
      title={label}
      onClick={onClick}
    >
      {icon}
    </button>
  );
}
