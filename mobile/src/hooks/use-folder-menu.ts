import { useState } from "react";

import type { FolderRef } from "@/components/folder-actions";

export function useFolderMenu() {
  const [target, setTarget] = useState<FolderRef | null>(null);
  const [visible, setVisible] = useState(false);

  return {
    target,
    visible,
    open: (folder: FolderRef) => {
      setTarget(folder);
      setVisible(true);
    },
    close: () => setVisible(false),
  };
}
