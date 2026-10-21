import * as SecureStore from "expo-secure-store";

export type Tokens = {
  accessToken: string;
  refreshToken: string;
};

const KEY = "tsundoku.session";

function isTokens(value: unknown): value is Tokens {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const candidate = value as Record<string, unknown>;
  return (
    typeof candidate.accessToken === "string" &&
    typeof candidate.refreshToken === "string"
  );
}

export const tokenStorage = {
  async load(): Promise<Tokens | null> {
    try {
      const raw = await SecureStore.getItemAsync(KEY);
      if (!raw) {
        return null;
      }

      const parsed: unknown = JSON.parse(raw);
      return isTokens(parsed) ? parsed : null;
    } catch (error) {
      console.warn("Could not read the saved session", error);
      return null;
    }
  },

  async save(tokens: Tokens): Promise<void> {
    await SecureStore.setItemAsync(KEY, JSON.stringify(tokens));
  },

  async clear(): Promise<void> {
    await SecureStore.deleteItemAsync(KEY);
  },
};
