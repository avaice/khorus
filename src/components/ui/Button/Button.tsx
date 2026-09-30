import type { ReactNode } from "react";

type ButtonProps = {
  children: ReactNode;
  onClick: () => void;
};

export function Button({ children, onClick }: ButtonProps) {
  return (
    <button type="button" className="button" onClick={onClick}>
      {children}
    </button>
  );
}
