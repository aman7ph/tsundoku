import axios from "axios";
import { API_URL } from "./config";
import type { Tokens } from "@/lib/token-storage";

const authApi = axios.create({ baseURL: API_URL, timeout: 15000 });

type TokenPairResponse = {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
};

function toTokens(data: TokenPairResponse): Tokens {
  return { accessToken: data.access_token, refreshToken: data.refresh_token };
}

export async function devLogin(): Promise<Tokens> {
  const { data } = await authApi.post<TokenPairResponse>("/auth/dev-login");
  return toTokens(data);
}

export async function refreshTokens(refreshToken: string): Promise<Tokens> {
  const { data } = await authApi.post<TokenPairResponse>("/auth/refresh", {
    refresh_token: refreshToken,
  });
  return toTokens(data);
}
