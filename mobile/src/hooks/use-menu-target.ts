import { useState } from "react";

export function useMenuTarget<T>() {
  const [target, setTarget] = useState<T | null>(null);
  const [visible, setVisible] = useState(false);

  return {
    target,
    visible,
    open: (item: T) => {
      setTarget(item);
      setVisible(true);
    },
    close: () => setVisible(false),
  };
}
