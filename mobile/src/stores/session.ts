import { create } from "zustand";

import { tokenStorage, type Tokens } from "@/lib/token-storage";

type SessionStatus = "loading" | "signedOut" | "signedIn";

type SessionState = {
  status: SessionStatus;
  tokens: Tokens | null;
  restore: () => Promise<void>;
  setTokens: (tokens: Tokens) => Promise<void>;
  signOut: () => Promise<void>;
};

export const useSessionStore = create<SessionState>()((set) => ({
  status: "loading",
  tokens: null,

  restore: async () => {
    const tokens = await tokenStorage.load();
    set({ tokens, status: tokens ? "signedIn" : "signedOut" });
  },

  setTokens: async (tokens) => {
    set({ tokens, status: "signedIn" });

    try {
      await tokenStorage.save(tokens);
    } catch (error) {
      console.warn("Could not save the session", error);
    }
  },

  signOut: async () => {
    set({ tokens: null, status: "signedOut" });

    try {
      await tokenStorage.clear();
    } catch (error) {
      console.warn("Could not clear the saved session", error);
    }
  },
}));
