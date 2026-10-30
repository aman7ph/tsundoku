import axios, { type InternalAxiosRequestConfig } from "axios";

import { refreshTokens } from "@/api/auth";
import { API_URL } from "@/api/config";
import { useSessionStore } from "@/stores/session";

export const api = axios.create({ baseURL: API_URL, timeout: 15000 });

type RetryableConfig = InternalAxiosRequestConfig & { retried?: boolean };

let refreshing: Promise<string> | null = null;

async function refreshAccessToken(): Promise<string> {
  const { tokens, setTokens, signOut } = useSessionStore.getState();

  if (!tokens) {
    throw new Error("Not signed in");
  }

  try {
    const fresh = await refreshTokens(tokens.refreshToken);
    await setTokens(fresh);
    return fresh.accessToken;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response?.status === 401) {
      await signOut();
    }
    throw error;
  }
}

api.interceptors.request.use((config) => {
  const accessToken = useSessionStore.getState().tokens?.accessToken;

  if (accessToken) {
    config.headers.set("Authorization", `Bearer ${accessToken}`);
  }

  return config;
});

api.interceptors.response.use(
  (response) => response,
  async (error: unknown) => {
    if (!axios.isAxiosError(error) || !error.config) {
      throw error;
    }

    const original = error.config as RetryableConfig;
    const signedIn = useSessionStore.getState().tokens !== null;

    if (error.response?.status !== 401 || original.retried || !signedIn) {
      throw error;
    }

    original.retried = true;
    refreshing ??= refreshAccessToken().finally(() => {
      refreshing = null;
    });

    const accessToken = await refreshing;
    original.headers.set("Authorization", `Bearer ${accessToken}`);
    return api(original);
  },
);
