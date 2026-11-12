import { create } from "zustand";

type RefreshState = { version: number; bump: () => void };

export const useRefreshStore = create<RefreshState>()((set) => ({
  version: 0,
  bump: () => set((state) => ({ version: state.version + 1 })),
}));
